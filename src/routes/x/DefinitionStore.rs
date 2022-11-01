use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;

use crate::components::routing::mod_route::RouteComponent;
use crate::components::routing::submodules::sub_body_data_type::BodyDataType;
use crate::components::routing::submodules::sub_next_condition_type::NextConditionType;
use crate::components::routing::submodules::sub_operation::Operation;
use crate::components::routing::submodules::sub_operation_type::OperationType;
use crate::components::routing::submodules::sub_ref_data::RefData;
use crate::data_converter::{convert_to_raw, RawPair};
use crate::utils::{auto_fetch_all_collections, auto_fetch_all_data, auto_fetch_all_mappings};

use super::global_block_order::GlobalBlockOrder;
use super::resolver::{resolve_conditions, resolve_operations, resolve_raw_data, resolve_ref_data};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionStore {
    pub block_name: String,
    pub ref_name: String,
    pub index: usize,
    pub data: DefinitionData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DefinitionData {
    NULL,
    UNDEFINED,
    BOOLEAN(bool),
    STRING(String),
    INTEGER(isize),
    FLOAT(f64),
    DATA(Vec<RawPair>),
}

impl DefinitionStore {
    pub fn add_definition(
        current_route: &RouteComponent,
        all_definitions: &mut Vec<DefinitionStore>,
        global_blocks: &Vec<GlobalBlockOrder>,
        project_id: &str,
        block_name: &str,
        index: usize,
        current_index: usize,
    ) -> Result<(), (usize, String)> {
        let mut actual_definition = DefinitionStore {
            block_name: block_name.to_string(),
            ref_name: String::from(""),
            index: index,
            data: DefinitionData::NULL,
        };

        if block_name == "FETCH" {
            let fetch_block = current_route.flow.fetchers[index].clone();

            let mappings = auto_fetch_all_mappings();
            let all_data = match auto_fetch_all_data(&mappings, &project_id, &fetch_block.ref_col) {
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

            let collection =
                match Collection::get(&all_collections, project_id, &fetch_block.ref_col) {
                    Ok(p) => p,
                    Err(_) => {
                        return Err((
                            404,
                            String::from("Error: No Collection with this collection_id found"),
                        ));
                    }
                };

            let current_data = Data::get_all(&all_data, project_id, &fetch_block.ref_col);
            let mut raw_pairs = Vec::<RawPair>::new();

            for data in current_data {
                match convert_to_raw(&data, &collection) {
                    Ok(rp) => {
                        raw_pairs.push(rp);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: fetch_block.local_name,
                index: index,
                data: DefinitionData::DATA(raw_pairs),
            };
        } else if block_name == "ASSIGN" {
            let assign_block = current_route.flow.assignments[index].clone();
            let res_condition = match resolve_conditions(
                &assign_block.conditions,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(c) => c,
                Err(e) => {
                    return Err(e);
                }
            };

            if res_condition {
                match resolve_operations(
                    &assign_block.operations,
                    global_blocks,
                    all_definitions,
                    current_index,
                ) {
                    Ok(d) => {
                        actual_definition = DefinitionStore {
                            block_name: block_name.to_string(),
                            ref_name: assign_block.local_name,
                            index: index,
                            data: d,
                        };
                    }
                    Err(e) => return Err(e),
                }
            }
        } else if block_name == "TEMPLATE" {
            let template_block = current_route.flow.templates[index].clone();
            let broken_template = template_block.template.split("{}").collect::<Vec<&str>>();
            let mut final_string = String::new();

            let res_condition = match resolve_conditions(
                &template_block.conditions,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(c) => c,
                Err(e) => {
                    return Err(e);
                }
            };

            if res_condition {
                for (i, str) in broken_template.iter().enumerate() {
                    let current_data = if template_block.data.len() == 0 {
                        match RefData::create(false, "STRING", "") {
                            Ok(rd) => rd,
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    } else if template_block.data.len() > i {
                        template_block.data[i].clone()
                    } else {
                        template_block.data[template_block.data.len() - 1].clone()
                    };

                    let current_value = if i == broken_template.len() - 1 {
                        DefinitionData::STRING(String::new())
                    } else {
                        match resolve_ref_data(
                            &current_data,
                            global_blocks,
                            all_definitions,
                            current_index,
                        ) {
                            Ok(d) => d,
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    };

                    match current_value {
                        DefinitionData::STRING(s) => {
                            final_string = format!("{}{}{}", final_string, str, s);
                        }
                        DefinitionData::INTEGER(i) => {
                            final_string = format!("{}{}{}", final_string, str, i);
                        }
                        DefinitionData::FLOAT(f) => {
                            final_string = format!("{}{}{}", final_string, str, f);
                        }
                        DefinitionData::BOOLEAN(b) => {
                            final_string = format!("{}{}{}", final_string, str, b);
                        }
                        _ => {
                            return Err((
                                500,
                                String::from("Error: Invalid data type for template"),
                            ));
                        }
                    }
                }

                actual_definition = DefinitionStore {
                    block_name: block_name.to_string(),
                    ref_name: template_block.local_name,
                    index: index,
                    data: DefinitionData::STRING(final_string),
                };
            }
        } else if block_name == "LOOP" {
            let loop_block = current_route.flow.loops[index].clone();

            let min = match resolve_ref_data(
                &loop_block.min,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(d) => d,
                Err(e) => {
                    return Err(e);
                }
            };

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: loop_block.local_name,
                index: index,
                data: min,
            };
        } else if block_name == "FILTER" {
            let filter_block = current_route.flow.filters[index].clone();
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
                DefinitionData::DATA(d) => d,
                _ => {
                    return Err((
                        500,
                        format!("Error: Invalid data type for '{}'", filter_block.ref_var),
                    ));
                }
            };

            let mut found_property = false;
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

            for raw_pair in current_data.iter() {
                if broken_property.len() < 2 {
                    for structure in raw_pair.structures.iter() {
                        if structure.id == broken_property[0] {
                            found_property = true;
                            break;
                        }
                    }
                } else {
                    for custom_structure in raw_pair.custom_structures.iter() {
                        if custom_structure.id == broken_property[0] {
                            for structure in custom_structure.structures.iter() {
                                if structure.id == broken_property[1] {
                                    found_property = true;
                                    break;
                                }
                            }
                        }

                        if found_property {
                            break;
                        }
                    }
                }

                if found_property {
                    break;
                }
            }

            if !found_property {
                return Err((
                    500,
                    format!(
                        "Error: Property '{}' not found in '{}'",
                        filter_block.ref_property, filter_block.ref_var
                    ),
                ));
            }

            let mut final_current_data = Vec::<RawPair>::new();
            let mut current_definition: DefinitionData;

            for raw_pair in current_data.iter() {
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
                            current_err = (500, String::from("Error: Invalid result data type while processing filter operations"));
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

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: filter_block.local_name,
                index: index,
                data: DefinitionData::DATA(final_current_data),
            };
        }

        if current_index >= all_definitions.len() {
            all_definitions.push(actual_definition);
        } else {
            all_definitions[current_index] = actual_definition;
        }

        Ok(())
    }

    pub fn update_definition_value(
        all_definitions: &mut Vec<DefinitionStore>,
        position: usize,
        data: DefinitionData,
    ) -> Result<(), (usize, String)> {
        if position > all_definitions.len() {
            return Err((500, String::from("Error: Invalid position for definition")));
        }

        all_definitions[position].data = data;

        Ok(())
    }

    pub fn get_raw_definition(
        all_definitions: &Vec<DefinitionStore>,
        ref_name: &str,
        index: usize,
    ) -> Option<DefinitionStore> {
        for definition in all_definitions {
            if definition.ref_name == ref_name && definition.index == index {
                return Some(definition.clone());
            }
        }

        None
    }

    pub fn to_string(all_definitions: &Vec<DefinitionStore>) -> Vec<String> {
        let mut definitions = Vec::<String>::new();
        for definition in all_definitions {
            definitions.push(format!(
                "{}/{}: {}",
                definition.block_name, definition.ref_name, definition.index
            ));
        }
        definitions
    }
}
