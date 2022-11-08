use crate::{
    components::{
        collection::Collection,
        data::Data,
        raw_pair::{CustomStructurePair, RawPair, StructurePair},
        routing::{
            blocks::update_block::UpdateBlock,
            submodules::{
                sub_body_data_type::BodyDataType, sub_condition::Condition,
                sub_condition_type::ConditionType, sub_next_condition_type::NextConditionType,
                sub_operation::Operation, sub_operation_type::OperationType, sub_ref_data::RefData,
            },
        },
    },
    routes::x_utils::{
        convertors::{
            convert_data_to_rawpair::data_to_rawpair,
            convert_definition_to_string::definition_to_string,
            convert_rawpair_to_data::rawpair_to_data,
        },
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::{resolve_conditions, resolve_operations, resolve_ref_data},
    },
    utils::{
        collection::auto_fetch_all_collections, data::auto_fetch_all_data,
        data::auto_save_all_data, mapping::auto_fetch_all_mappings,
    },
};

pub fn define_update(
    update_block: UpdateBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
    project_id: &str,
) -> Result<DefinitionData, (usize, String)> {
    let res_condition = match resolve_conditions(
        &update_block.conditions,
        global_blocks,
        all_definitions,
        current_index,
    ) {
        Ok(c) => c,
        Err(e) => {
            return Err(e);
        }
    };

    if !res_condition {
        return Ok(DefinitionData::NULL);
    }

    let mappings = auto_fetch_all_mappings();
    let all_data = match auto_fetch_all_data(&mappings, &project_id, &update_block.ref_col) {
        Ok(d) => d,
        _ => {
            return Err((500, String::from("Error: Failed fetching data")));
        }
    };

    let all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return Err((500, String::from("Error: Failed fetching collections")));
        }
    };

    let collection = match Collection::get(&all_collections, project_id, &update_block.ref_col) {
        Ok(p) => p,
        Err(_) => {
            return Err((
                404,
                String::from("Error: No Collection with this collection_id found"),
            ));
        }
    };

    let current_data = Data::get_all(&all_data, project_id, &update_block.ref_col);
    let mut current_raw_pairs = Vec::<RawPair>::new();

    for data in current_data {
        match data_to_rawpair(&data, &collection) {
            Ok(rp) => {
                current_raw_pairs.push(rp);
            }
            Err(e) => {
                return Err(e);
            }
        };
    }

    let mut final_current_data = Vec::<RawPair>::new();
    let mut current_definition: DefinitionData;

    for raw_pair in current_raw_pairs.iter() {
        let mut is_raw_pair_valid = false;

        if update_block.targets.len() < 1 {
            final_current_data.push(raw_pair.clone());
        }

        let mut current_err: (usize, String) = (0, String::new());

        for target in update_block.targets.iter() {
            let mut current_value = String::new();
            let mut current_rtype = BodyDataType::STRING;

            let broken_property = target.field.split(".").collect::<Vec<&str>>();
            if broken_property.len() > 2 || broken_property[0].trim().len() < 1 {
                return Err((
                    500,
                    format!("Error: Invalid property '{}' in update", target.field),
                ));
            }

            if broken_property.len() < 2 {
                for structure in raw_pair.structures.iter() {
                    if structure.id == broken_property[0] {
                        current_value = structure.value.clone();
                        current_rtype = BodyDataType::from(&structure.rtype);
                        break;
                    }
                }
            } else {
                for custom_structure in raw_pair.custom_structures.iter() {
                    if custom_structure.id == broken_property[0] {
                        for structure in custom_structure.structures.iter() {
                            if structure.id == broken_property[1] {
                                current_value = structure.value.clone();
                                current_rtype = BodyDataType::from(&structure.rtype);
                                break;
                            }
                        }
                    }

                    if current_value.len() > 0 {
                        break;
                    }
                }
            }

            match RefData::create(
                false,
                &BodyDataType::to(current_rtype.clone()),
                &current_value,
            ) {
                Ok(left) => {
                    let mut current_conditions = Vec::<Condition>::new();
                    for condition in target.conditions.iter() {
                        Condition::create(
                            &mut current_conditions,
                            left.clone(),
                            condition.right.clone(),
                            &ConditionType::to(condition.condition_type.clone()),
                            condition.not,
                            &NextConditionType::to(condition.next.clone()),
                        );
                    }

                    match resolve_conditions(
                        &current_conditions,
                        global_blocks,
                        all_definitions,
                        current_index,
                    ) {
                        Ok(b) => {
                            current_definition = DefinitionData::BOOLEAN(b);
                        }
                        Err(e) => {
                            current_definition = DefinitionData::UNDEFINED;
                            current_err = e;
                        }
                    }
                }
                Err(e) => {
                    current_definition = DefinitionData::UNDEFINED;
                    current_err = e;
                }
            }

            if current_definition == DefinitionData::UNDEFINED && current_err.0 > 0 {
                return Err((
                    current_err.0,
                    format!(
                        "Error: Failed processing Update: '{}'",
                        current_err.1.split("Error: ").collect::<Vec<&str>>()[1]
                    ),
                ));
            } else {
                match current_definition {
                    DefinitionData::BOOLEAN(b) => {
                        is_raw_pair_valid = b;
                    }
                    _ => {
                        return Err((
                            500,
                            format!("Error: Invalid data type for last target condition in Update"),
                        ));
                    }
                }
            }

            if !is_raw_pair_valid {
                break;
            }
        }

        if is_raw_pair_valid {
            final_current_data.push(raw_pair.clone());
        }
    }

    let mut final_processed_current_data = Vec::<RawPair>::new();
    let broken_property = update_block.ref_property.split(".").collect::<Vec<&str>>();

    for raw_pair in final_current_data.iter() {
        if update_block.ref_property.trim().len() < 1 {
            if let Some(set) = update_block.set.clone() {
                if let Ok(rd) =
                    resolve_ref_data(&set, global_blocks, all_definitions, current_index)
                {
                    match rd {
                        DefinitionData::DATA(d) => {
                            final_processed_current_data.push(d.clone());
                        }
                        _ => {
                            return Err((
                                500,
                                format!("Error: Invalid data type for set in Update"),
                            ));
                        }
                    }
                }
            }
        } else {
            let mut current_value = String::new();
            let mut current_rtype = BodyDataType::STRING;

            if broken_property.len() < 2 {
                for structure in raw_pair.structures.iter() {
                    if structure.id == broken_property[0] {
                        current_value = structure.value.clone();
                        current_rtype = BodyDataType::from(&structure.rtype);
                        break;
                    }
                }
            } else {
                for custom_structure in raw_pair.custom_structures.iter() {
                    if custom_structure.id == broken_property[0] {
                        for structure in custom_structure.structures.iter() {
                            if structure.id == broken_property[1] {
                                current_value = structure.value.clone();
                                current_rtype = BodyDataType::from(&structure.rtype);
                                break;
                            }
                        }
                    }

                    if current_value.len() > 0 {
                        break;
                    }
                }
            }

            if let Some(set) = update_block.set.clone() {
                if let Ok(rd) =
                    resolve_ref_data(&set, global_blocks, all_definitions, current_index)
                {
                    match definition_to_string(rd, "", "set in Update", false) {
                        Ok(s) => {
                            current_value = s;
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
            }

            if let Some(filter) = update_block.filter.clone() {
                if current_rtype.clone() == BodyDataType::ARRAY
                    && filter.right.rtype != BodyDataType::ARRAY
                {
                    let mut final_current_value = String::new();

                    for c_val in current_value.split(",") {
                        match resolve_operations(
                            &vec![Operation {
                                left: RefData {
                                    ref_var: false,
                                    rtype: filter.right.rtype.clone(),
                                    data: c_val.trim().to_string(),
                                },
                                right: filter.right.clone(),
                                operation_type: filter.operation_type.clone(),
                                next: filter.next.clone(),
                                not: filter.not,
                            }],
                            global_blocks,
                            all_definitions,
                            current_index,
                        ) {
                            Ok(def) => match def {
                                DefinitionData::BOOLEAN(b) => {
                                    if b {
                                        if final_current_value.len() > 0 {
                                            final_current_value =
                                                format!("{},{}", final_current_value, c_val);
                                        } else {
                                            final_current_value = c_val.to_string();
                                        }
                                    }
                                }
                                _ => {
                                    return Err((
                                        500,
                                        format!("Error: Invalid data type for filter in Update"),
                                    ));
                                }
                            },
                            _ => {}
                        }
                    }

                    current_value = final_current_value;
                }
            }

            if let Some(add) = update_block.add.clone() {
                if let Ok(rd) =
                    resolve_ref_data(&add, global_blocks, all_definitions, current_index)
                {
                    if current_rtype == BodyDataType::ARRAY {
                        match definition_to_string(rd, "", "add in Update", false) {
                            Ok(s) => {
                                if current_value.len() > 0 {
                                    current_value = format!("{},{}", current_value, s);
                                } else {
                                    current_value = s;
                                }
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    } else {
                        match resolve_operations(
                            &vec![Operation {
                                left: RefData {
                                    ref_var: false,
                                    rtype: current_rtype,
                                    data: current_value,
                                },
                                right: add,
                                operation_type: OperationType::ADDITION,
                                next: NextConditionType::NONE,
                                not: false,
                            }],
                            global_blocks,
                            all_definitions,
                            current_index,
                        ) {
                            Ok(res) => {
                                match definition_to_string(res, "", "add in Update", false) {
                                    Ok(s) => {
                                        current_value = s;
                                    }
                                    Err(e) => {
                                        return Err(e);
                                    }
                                }
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            }

            let mut final_structures = Vec::<StructurePair>::new();
            let mut final_custom_structures = Vec::<CustomStructurePair>::new();

            for structure in raw_pair.structures.iter() {
                if broken_property.len() < 2 && structure.id == broken_property[0] {
                    final_structures.push(StructurePair {
                        id: structure.id.clone(),
                        value: current_value.clone(),
                        rtype: structure.rtype.clone(),
                    });
                } else {
                    final_structures.push(structure.clone());
                }
            }

            for custom_structure in raw_pair.custom_structures.iter() {
                if broken_property.len() > 1 && custom_structure.id == broken_property[0] {
                    let mut final_custom_structure_structures = Vec::<StructurePair>::new();

                    for structure in custom_structure.structures.iter() {
                        if structure.id == broken_property[1] {
                            final_custom_structure_structures.push(StructurePair {
                                id: structure.id.clone(),
                                value: current_value.clone(),
                                rtype: structure.rtype.clone(),
                            });
                        } else {
                            final_custom_structure_structures.push(structure.clone());
                        }
                    }

                    final_custom_structures.push(CustomStructurePair {
                        id: custom_structure.id.clone(),
                        structures: final_custom_structure_structures,
                    });
                } else {
                    final_custom_structures.push(custom_structure.clone());
                }
            }

            final_processed_current_data.push(RawPair {
                data_id: raw_pair.data_id.clone(),
                structures: final_structures,
                custom_structures: final_custom_structures,
                published: raw_pair.published,
            });
        }
    }

    let mut final_data_to_save = Vec::<RawPair>::new();

    let mut all_data_ids = Vec::<String>::new();
    for raw_pair in final_processed_current_data.iter() {
        if raw_pair.data_id != "" {
            all_data_ids.push(raw_pair.data_id.clone());
        }
    }

    let mut current_index = 0;
    for raw_pair in current_raw_pairs {
        if all_data_ids.contains(&raw_pair.data_id) {
            if current_index < final_processed_current_data.len() {
                final_data_to_save.push(final_processed_current_data[current_index].clone());
                current_index += 1;
            }
        } else {
            final_data_to_save.push(raw_pair.clone());
        }
    }

    if update_block.save {
        let mut final_data_to_save_converted = Vec::<Data>::new();
        for raw_pair in final_data_to_save.iter() {
            if let Err(e) = rawpair_to_data(
                &mut final_data_to_save_converted,
                &collection,
                raw_pair,
                true,
            ) {
                return Err(e);
            }
        }

        match auto_save_all_data(
            &mappings,
            &project_id,
            &update_block.ref_col,
            &final_data_to_save_converted,
        ) {
            Err(e) => {
                return Err((500, e));
            }
            _ => {}
        }
    }

    let mut final_data = Vec::<DefinitionData>::new();
    for raw_pair in final_data_to_save.iter() {
        final_data.push(DefinitionData::DATA(raw_pair.clone()));
    }

    Ok(DefinitionData::ARRAY(final_data))
}
