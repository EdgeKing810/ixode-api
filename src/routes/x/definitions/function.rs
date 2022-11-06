use chrono::prelude::*;
use uuid::Uuid;

use crate::{
    components::routing::{
        blocks::function_block::FunctionBlock, submodules::sub_function_list::FunctionList,
    },
    middlewares::paginate::paginate,
    routes::x_utils::{
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::resolve_ref_data,
    },
};

pub fn define_function(
    function_block: FunctionBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
    let final_data: DefinitionData;

    match function_block.func.id {
        FunctionList::GENERATE_TIMESTAMP => {
            let timestamp = Utc::now().to_string();
            final_data = DefinitionData::STRING(timestamp);
        }
        FunctionList::V4 => {
            let uuid = Uuid::new_v4().to_string();
            final_data = DefinitionData::STRING(uuid);
        }
        FunctionList::PAGINATE => {
            if function_block.func.params.len() < 1 {
                return Err((
                    500,
                    format!("Error: Invalid number of parameters for PAGINATE function"),
                ));
            }

            let data_to_paginate = match resolve_ref_data(
                &function_block.func.params[0],
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(d) => match d {
                    DefinitionData::ARRAY(a) => a,
                    _ => {
                        return Err((
                                    500,
                                    format!("Error: Invalid data type for the 'data' param of the PAGINATE function"),
                                ));
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            };

            let limit = if function_block.func.params.len() > 1 {
                match resolve_ref_data(
                    &function_block.func.params[1],
                    global_blocks,
                    all_definitions,
                    current_index,
                ) {
                    Ok(d) => match d {
                        DefinitionData::INTEGER(i) => i as usize,
                        DefinitionData::FLOAT(f) => f as usize,
                        DefinitionData::STRING(s) => {
                            if let Ok(i) = s.parse::<usize>() {
                                i
                            } else {
                                return Err((
                                            500,
                                            format!("Error: Invalid data type for the 'limit' param of the PAGINATE function"),
                                        ));
                            }
                        }
                        _ => {
                            return Err((
                                        500,
                                        format!("Error: Invalid data type for the 'limit' param of the PAGINATE function"),
                                    ));
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else {
                10
            };

            let offset = if function_block.func.params.len() > 2 {
                match resolve_ref_data(
                    &function_block.func.params[2],
                    global_blocks,
                    all_definitions,
                    current_index,
                ) {
                    Ok(d) => match d {
                        DefinitionData::INTEGER(i) => i as usize,
                        DefinitionData::FLOAT(f) => f as usize,
                        DefinitionData::STRING(s) => {
                            if let Ok(i) = s.parse::<usize>() {
                                i
                            } else {
                                return Err((
                                            500,
                                            format!("Error: Invalid data type for the 'offset' param of the PAGINATE function"),
                                        ));
                            }
                        }
                        _ => {
                            return Err((
                                        500,
                                        format!("Error: Invalid data type for the 'offset' param of the PAGINATE function"),
                                    ));
                        }
                    },
                    Err(e) => {
                        return Err(e);
                    }
                }
            } else {
                0
            };

            let paginated_data = paginate(data_to_paginate, limit, offset);
            final_data = DefinitionData::ARRAY(paginated_data);
        }
    }

    Ok(final_data)
}
