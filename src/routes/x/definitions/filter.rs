use crate::{
    components::routing::{
        blocks::filter_block::FilterBlock,
        submodules::{
            sub_body_data_type::BodyDataType, sub_next_condition_type::NextConditionType,
            sub_operation::Operation, sub_operation_type::OperationType, sub_ref_data::RefData,
        },
    },
    data_converter::RawPair,
    routes::x_utils::{
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::{resolve_operations, resolve_raw_data},
    },
};

pub fn define_filter(
    filter_block: FilterBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
    let current_ref_data = match resolve_raw_data(
        &filter_block.ref_var,
        &global_blocks,
        &all_definitions,
        current_index,
    ) {
        Ok(rd) => rd,
        Err(e) => {
            return Err(e);
        }
    };

    let current_data = match current_ref_data {
        DefinitionData::ARRAY(a) => a,
        _ => {
            return Err((
                500,
                format!("Error: Invalid data type for '{}'", filter_block.ref_var),
            ));
        }
    };

    let broken_property = filter_block.ref_property.split(".").collect::<Vec<&str>>();
    if broken_property.len() > 2 || broken_property[0].trim().len() < 1 {
        return Err((
            500,
            format!(
                "Error: Invalid property '{}' in filter",
                filter_block.ref_property
            ),
        ));
    }

    let mut final_current_data = Vec::<RawPair>::new();
    let mut current_definition: DefinitionData;

    for raw_pair_data in current_data.iter() {
        let raw_pair = match raw_pair_data {
            DefinitionData::DATA(d) => d,
            _ => {
                continue;
            }
        };

        if filter_block.filters.len() < 1 {
            final_current_data.push(raw_pair.clone());
        }

        let mut current_value = String::new();
        if broken_property.len() < 2 {
            for structure in raw_pair.structures.iter() {
                if structure.id == broken_property[0] {
                    current_value = structure.value.clone();
                    break;
                }
            }
        } else {
            for custom_structure in raw_pair.custom_structures.iter() {
                if custom_structure.id == broken_property[0] {
                    for structure in custom_structure.structures.iter() {
                        if structure.id == broken_property[1] {
                            current_value = structure.value.clone();
                            break;
                        }
                    }
                }

                if current_value.len() > 0 {
                    break;
                }
            }
        }

        if current_value.len() < 1 {
            continue;
        }

        current_definition = DefinitionData::STRING(current_value);
        let mut current_operations: Vec<Operation>;
        let mut current_err: (usize, String) = (0, String::new());

        for filter in filter_block.filters.iter() {
            let mut rtype = BodyDataType::to(filter.right.rtype.clone());
            if rtype == String::from("OTHER") {
                rtype = String::from("STRING");
            }

            let data = match current_definition {
                DefinitionData::STRING(s) => s,
                DefinitionData::INTEGER(i) => i.to_string(),
                DefinitionData::FLOAT(f) => f.to_string(),
                DefinitionData::BOOLEAN(b) => b.to_string(),
                _ => {
                    current_definition = DefinitionData::UNDEFINED;
                    current_err = (
                        500,
                        String::from(
                            "Error: Invalid result data type while processing filter operations",
                        ),
                    );
                    break;
                }
            };

            match RefData::create(false, &rtype, &data) {
                Ok(left) => {
                    current_operations = Vec::<Operation>::new();
                    Operation::create(
                        &mut current_operations,
                        left,
                        filter.right.clone(),
                        &OperationType::to(filter.operation_type.clone()),
                        filter.not,
                        &NextConditionType::to(filter.next.clone()),
                    );

                    match resolve_operations(
                        &current_operations,
                        global_blocks,
                        all_definitions,
                        current_index,
                    ) {
                        Ok(d) => {
                            current_definition = d;
                        }
                        Err(e) => {
                            current_definition = DefinitionData::UNDEFINED;
                            current_err = e;
                            break;
                        }
                    }
                }
                Err(e) => {
                    current_definition = DefinitionData::UNDEFINED;
                    current_err = e;
                    break;
                }
            }
        }

        if current_definition == DefinitionData::UNDEFINED && current_err.0 > 0 {
            return Err((
                current_err.0,
                format!(
                    "Error: Failed processing Filter: '{}'",
                    current_err.1.split("Error: ").collect::<Vec<&str>>()[1]
                ),
            ));
        } else {
            match current_definition {
                DefinitionData::BOOLEAN(b) => {
                    if b {
                        final_current_data.push(raw_pair.clone());
                    }
                }
                _ => {
                    return Err((
                        500,
                        format!("Error: Invalid data type for last operation in Filter"),
                    ));
                }
            }
        }
    }

    let mut final_data = Vec::<DefinitionData>::new();
    for raw_pair in final_current_data.iter() {
        final_data.push(DefinitionData::DATA(raw_pair.clone()));
    }

    Ok(DefinitionData::ARRAY(final_data))
}
