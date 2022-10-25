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
    name: String,
}

impl GlobalBlockOrder {
    pub fn process_blocks(
        current_route: &RouteComponent,
        global_blocks: &mut Vec<GlobalBlockOrder>,
    ) {
        let mut current_global_index: u32 = 0;
        let mut current_block_name: &str;

        loop {
            current_block_name = "";

            if FetchBlock::exist(&current_route.flow.fetchers, current_global_index) {
                current_block_name = "FETCH";
            } else if AssignmentBlock::exist(&current_route.flow.assignments, current_global_index)
            {
                current_block_name = "ASSIGN";
            } else if TemplateBlock::exist(&current_route.flow.templates, current_global_index) {
                current_block_name = "TEMPLATE";
            } else if ConditionBlock::exist(&current_route.flow.conditions, current_global_index) {
                current_block_name = "CONDITION";
            } else if LoopBlock::exist(&current_route.flow.loops, current_global_index) {
                current_block_name = "LOOP";
            } else if FilterBlock::exist(&current_route.flow.filters, current_global_index) {
                current_block_name = "FILTER";
            } else if PropertyBlock::exist(&current_route.flow.properties, current_global_index) {
                current_block_name = "PROPERTY";
            } else if FunctionBlock::exist(&current_route.flow.functions, current_global_index) {
                current_block_name = "FUNCTION";
            } else if ObjectBlock::exist(&current_route.flow.objects, current_global_index) {
                current_block_name = "OBJECT";
            } else if UpdateBlock::exist(&current_route.flow.updates, current_global_index) {
                current_block_name = "UPDATE";
            } else if CreateBlock::exist(&current_route.flow.creates, current_global_index) {
                current_block_name = "CREATE";
            } else if ReturnBlock::exist(&current_route.flow.returns, current_global_index) {
                current_block_name = "RETURN";
            }

            if current_block_name.len() > 0 {
                global_blocks.push(GlobalBlockOrder {
                    name: current_block_name.to_string(),
                });
            } else {
                break;
            }

            current_global_index += 1;
        }
    }

    pub fn to_string(global_blocks: &Vec<GlobalBlockOrder>) -> Vec<String> {
        let mut block_names = Vec::<String>::new();
        for block in global_blocks {
            block_names.push(block.name.clone());
        }
        block_names
    }
}
