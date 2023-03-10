use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::blocks::assignment_block::AssignmentBlock;
use crate::components::routing::blocks::condition_block::ConditionBlock;
use crate::components::routing::blocks::create_block::CreateBlock;
use crate::components::routing::blocks::fetch_block::FetchBlock;
use crate::components::routing::blocks::filter_block::FilterBlock;
use crate::components::routing::blocks::function_block::FunctionBlock;
use crate::components::routing::blocks::loop_block::LoopBlock;
use crate::components::routing::blocks::object_block::ObjectBlock;
use crate::components::routing::blocks::property_block::PropertyBlock;
use crate::components::routing::blocks::return_block::ReturnBlock;
use crate::components::routing::blocks::template_block::TemplateBlock;
use crate::components::routing::blocks::update_block::UpdateBlock;

use crate::components::routing::mod_route::RouteComponent;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GlobalBlockOrder {
    pub index: usize,
    pub block_index: isize,
    pub name: String,
    pub ref_name: String,
}

impl GlobalBlockOrder {
    pub fn process_blocks(
        current_route: &RouteComponent,
        global_blocks: &mut Vec<GlobalBlockOrder>,
    ) {
        for (i, body_data) in current_route.body.iter().enumerate() {
            global_blocks.push(GlobalBlockOrder {
                index: i,
                block_index: -1,
                name: String::from("BODY"),
                ref_name: body_data.id.clone(),
            });
        }

        if let Some(params) = &current_route.params {
            for (i, pair) in params.pairs.iter().enumerate() {
                global_blocks.push(GlobalBlockOrder {
                    index: i,
                    block_index: -1,
                    name: String::from("PARAM"),
                    ref_name: pair.id.clone(),
                });
            }
        }

        let mut current_global_index: u32 = 0;
        let mut current_block_name: &str;
        let mut current_index_position: usize = 0;
        let mut current_ref_name: String;
        let mut current_block_index: isize;

        let mut indexes: Vec<usize> = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        loop {
            current_block_name = "";
            current_ref_name = String::from("");
            current_block_index = -1;

            if let Some(block) = FetchBlock::get(&current_route.flow.fetchers, current_global_index)
            {
                current_block_name = "FETCH";
                current_index_position = 0;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                AssignmentBlock::get(&current_route.flow.assignments, current_global_index)
            {
                current_block_name = "ASSIGN";
                current_index_position = 1;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                TemplateBlock::get(&current_route.flow.templates, current_global_index)
            {
                current_block_name = "TEMPLATE";
                current_index_position = 2;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                ConditionBlock::get(&current_route.flow.conditions, current_global_index)
            {
                current_block_name = "CONDITION";
                current_index_position = 3;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                LoopBlock::get(&current_route.flow.loops, current_global_index)
            {
                current_block_name = "LOOP";
                current_index_position = 4;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                FilterBlock::get(&current_route.flow.filters, current_global_index)
            {
                current_block_name = "FILTER";
                current_index_position = 5;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                PropertyBlock::get(&current_route.flow.properties, current_global_index)
            {
                current_block_name = "PROPERTY";
                current_index_position = 6;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                FunctionBlock::get(&current_route.flow.functions, current_global_index)
            {
                current_block_name = "FUNCTION";
                current_index_position = 7;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                ObjectBlock::get(&current_route.flow.objects, current_global_index)
            {
                current_block_name = "OBJECT";
                current_index_position = 8;
                current_ref_name = block.local_name;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                UpdateBlock::get(&current_route.flow.updates, current_global_index)
            {
                current_block_name = "UPDATE";
                current_index_position = 9;
                current_ref_name = format!(
                    "update_{}_{}",
                    block.ref_col, indexes[current_index_position]
                );
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                CreateBlock::get(&current_route.flow.creates, current_global_index)
            {
                current_block_name = "CREATE";
                current_index_position = 10;
                current_block_index = block.block_index as isize;
            } else if let Some(block) =
                ReturnBlock::get(&current_route.flow.returns, current_global_index)
            {
                current_block_name = "RETURN";
                current_index_position = 11;
                current_block_index = block.block_index as isize;
            }

            if current_block_name.len() > 0 {
                global_blocks.push(GlobalBlockOrder {
                    index: indexes[current_index_position],
                    block_index: current_block_index,
                    name: current_block_name.to_string(),
                    ref_name: current_ref_name,
                });
                indexes[current_index_position] += 1;
            } else {
                break;
            }

            current_global_index += 1;
        }
    }

    pub fn to_string(global_blocks: &Vec<GlobalBlockOrder>) -> Vec<String> {
        let mut block_names = Vec::<String>::new();
        for block in global_blocks {
            block_names.push(format!(
                "{}.{}: {}",
                block.name, block.index, block.ref_name
            ));
        }
        block_names
    }

    pub fn get_ref_index(
        global_blocks: &Vec<GlobalBlockOrder>,
        ref_name: &str,
        current_index: usize,
    ) -> Result<(usize, String), (usize, String)> {
        let mut potential_result: Option<(usize, String)> = None;

        for (i, block) in global_blocks.iter().enumerate() {
            if i > current_index {
                break;
            }

            if block.ref_name == ref_name && block.ref_name.len() > 0 {
                potential_result = Some((block.index, block.name.clone()));
            }
        }

        if let Some(res) = potential_result {
            return Ok(res);
        }

        return Err((
            0,
            format!("Error: Referencing undefined variable: {}", ref_name),
        ));
    }
}
