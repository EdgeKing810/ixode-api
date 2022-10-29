use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;

use crate::components::routing::mod_route::RouteComponent;
use crate::data_converter::{convert_to_raw, RawPair};
use crate::utils::{auto_fetch_all_collections, auto_fetch_all_data, auto_fetch_all_mappings};

use super::global_block_order::GlobalBlockOrder;
use super::resolver::{resolve_conditions, resolve_operations};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionStore {
    pub definition_type: String,
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

            all_definitions.push(DefinitionStore {
                definition_type: String::from("DATA"),
                block_name: block_name.to_string(),
                ref_name: fetch_block.local_name,
                index: index,
                data: DefinitionData::DATA(raw_pairs),
            });

            return Ok(());
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
                        all_definitions.push(DefinitionStore {
                            definition_type: String::from("STRING"),
                            block_name: block_name.to_string(),
                            ref_name: assign_block.local_name,
                            index: index,
                            data: d,
                        });
                    }
                    Err(e) => return Err(e),
                }

                return Ok(());
            }
        }

        all_definitions.push(DefinitionStore {
            definition_type: String::from("EMPTY"),
            block_name: block_name.to_string(),
            ref_name: String::from(""),
            index: index,
            data: DefinitionData::NULL,
        });

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
                "{}/{} {}: {}",
                definition.block_name,
                definition.definition_type,
                definition.ref_name,
                definition.index
            ));
        }
        definitions
    }
}
