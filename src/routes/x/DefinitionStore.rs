use rocket::serde::{Deserialize, Serialize};

use serde_json::Value;

use crate::components::raw_pair::RawPair;
use crate::components::routing::mod_route::RouteComponent;
use crate::routes::x::LocalParamData;

use super::definitions::define_assign::define_assign;
use super::definitions::define_body::define_body;
use super::definitions::define_create::define_create;
use super::definitions::define_fetch::define_fetch;
use super::definitions::define_filter::define_filter;
use super::definitions::define_function::define_function;
use super::definitions::define_loop::define_loop;
use super::definitions::define_object::define_object;
use super::definitions::define_param::define_param;
use super::definitions::define_property::define_property;
use super::definitions::define_template::define_template;
use super::definitions::define_update::define_update;

use super::global_block_order::GlobalBlockOrder;

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
    ARRAY(Vec<DefinitionData>),
    DATA(RawPair),
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
        actual_body: &Value,
        all_params: &Vec<LocalParamData>,
    ) -> Result<(), (usize, String)> {
        let mut actual_definition = DefinitionStore {
            block_name: block_name.to_string(),
            ref_name: String::from(""),
            index: index,
            data: DefinitionData::NULL,
        };

        if block_name == "BODY" {
            let current_body = current_route.body[index].clone();
            actual_definition.ref_name = current_body.id.clone();
            match define_body(actual_body, current_body) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "PARAM" {
            if let Some(params) = &current_route.params {
                match define_param(params, all_params, index) {
                    Ok(d) => {
                        if d != DefinitionData::NULL {
                            actual_definition.ref_name = params.pairs[index].id.clone();
                        }
                        actual_definition.data = d;
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
        } else if block_name == "FETCH" {
            let fetch_block = current_route.flow.fetchers[index].clone();

            actual_definition.ref_name = fetch_block.local_name.clone();
            match define_fetch(fetch_block, project_id) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "ASSIGN" {
            let assign_block = current_route.flow.assignments[index].clone();

            actual_definition.ref_name = assign_block.local_name.clone();
            match define_assign(assign_block, global_blocks, all_definitions, current_index) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "TEMPLATE" {
            let template_block = current_route.flow.templates[index].clone();

            actual_definition.ref_name = template_block.local_name.clone();
            match define_template(
                template_block,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "LOOP" {
            let loop_block = current_route.flow.loops[index].clone();

            actual_definition.ref_name = loop_block.local_name.clone();
            match define_loop(loop_block, global_blocks, all_definitions, current_index) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "FILTER" {
            let filter_block = current_route.flow.filters[index].clone();

            actual_definition.ref_name = filter_block.local_name.clone();
            match define_filter(filter_block, global_blocks, all_definitions, current_index) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "PROPERTY" {
            let property_block = current_route.flow.properties[index].clone();

            actual_definition.ref_name = property_block.local_name.clone();
            match define_property(
                property_block,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "FUNCTION" {
            let function_block = current_route.flow.functions[index].clone();

            actual_definition.ref_name = function_block.local_name.clone();
            match define_function(
                function_block,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "OBJECT" {
            let object_block = current_route.flow.objects[index].clone();

            actual_definition.ref_name = object_block.local_name.clone();
            match define_object(object_block, global_blocks, all_definitions, current_index) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "UPDATE" {
            let update_block = current_route.flow.updates[index].clone();

            actual_definition.ref_name = format!("update_{}_{}", update_block.ref_col, index);
            match define_update(
                update_block,
                global_blocks,
                all_definitions,
                current_index,
                project_id,
            ) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        } else if block_name == "CREATE" {
            let create_block = current_route.flow.creates[index].clone();

            actual_definition.ref_name = format!("create_{}_{}", create_block.ref_col, index);
            match define_create(
                create_block,
                global_blocks,
                all_definitions,
                current_index,
                project_id,
            ) {
                Ok(d) => {
                    actual_definition.data = d;
                }
                Err(e) => {
                    return Err(e);
                }
            }
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
