use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::mod_route::RouteComponent;

use super::{
    definition_store::{DefinitionData, DefinitionStore},
    global_block_order::GlobalBlockOrder,
    resolver::resolve_ref_data,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopObject {
    pub start_index: usize,
    pub end_index: usize,
    pub ref_var: String,
}

impl LoopObject {
    pub fn detect_loops(
        global_blocks: &Vec<GlobalBlockOrder>,
        current_block: &GlobalBlockOrder,
    ) -> Result<Vec<LoopObject>, (usize, String)> {
        let mut loop_objects = Vec::<LoopObject>::new();

        let block_index = current_block.block_index;
        let mut start_index = 0;
        let mut is_processing_block = false;
        let mut loop_var = String::new();

        for (i, block) in global_blocks.iter().enumerate() {
            if is_processing_block && (block.name == "LOOP" || block.block_index > block_index) {
                is_processing_block = false;
                loop_objects.push(LoopObject {
                    start_index: start_index,
                    end_index: i,
                    ref_var: loop_var.clone(),
                });
                start_index = 0;
            }

            if block.block_index == block_index && block.name == "LOOP" {
                is_processing_block = true;
                start_index = i;
                loop_var = block.ref_name.clone();
            }
        }

        if is_processing_block {
            loop_objects.push(LoopObject {
                start_index: start_index,
                end_index: global_blocks.len(),
                ref_var: loop_var,
            });
        }

        Ok(loop_objects)
    }

    pub fn check_completed(
        global_blocks: &Vec<GlobalBlockOrder>,
        all_definitions: &Vec<DefinitionStore>,
        current_route: &RouteComponent,
        current_loop: &LoopObject,
    ) -> Result<bool, (usize, String)> {
        let mut global_loop_index = 0;
        let current = match GlobalBlockOrder::get_ref_index(
            global_blocks,
            &current_loop.ref_var,
            current_loop.start_index + 1,
        ) {
            Ok(ri) => {
                global_loop_index = ri.0;
                let current_definition = DefinitionStore::get_raw_definition(
                    all_definitions,
                    &current_loop.ref_var,
                    ri.0,
                );

                if let Some(def) = current_definition {
                    def.data.clone()
                } else {
                    DefinitionData::NULL
                }
            }
            Err(_) => DefinitionData::UNDEFINED,
        };

        let max = match resolve_ref_data(
            &current_route.flow.loops[global_loop_index].max.clone(),
            global_blocks,
            all_definitions,
            current_loop.start_index + 1,
        ) {
            Ok(d) => d,
            Err(e) => {
                return Err(e);
            }
        };

        match (current, max) {
            (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                return Ok(l >= r);
            }
            (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                return Ok(l >= r);
            }
            (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                return Ok((l as f64) >= r);
            }
            (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                return Ok(l >= (r as f64));
            }
            (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                return Ok(l.len() >= (r as usize));
            }
            (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                return Ok((l as usize) >= r.len());
            }
            (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                return Ok(l.len() >= (r as usize));
            }
            (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                return Ok((l as usize) >= r.len());
            }
            (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                return Ok(l.len() >= r.len());
            }
            _ => {
                return Err((500, format!("Error: Cannot compare 'current' and 'max'",)));
            }
        }
    }
}
