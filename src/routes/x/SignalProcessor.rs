use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::mod_route::RouteComponent;
use crate::components::routing::submodules::sub_condition_action::ConditionAction;

use super::definition_store::DefinitionStore;
use super::global_block_order::GlobalBlockOrder;
use super::resolver::resolve_conditions;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Signal {
    NONE,
    BREAK,
    CONTINUE,
    FAIL(usize, String),
}

pub fn obtain_signal(
    current_route: &RouteComponent,
    all_definitions: &mut Vec<DefinitionStore>,
    global_blocks: &Vec<GlobalBlockOrder>,
    block_name: &str,
    index: usize,
    current_index: usize,
) -> Result<Signal, (usize, String)> {
    if block_name == "CONDITION" {
        let condition_block = current_route.flow.conditions[index].clone();
        let res_condition = match resolve_conditions(
            &condition_block.conditions,
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
            match condition_block.action {
                ConditionAction::FAIL => {
                    if let Some(f_obj) = condition_block.fail {
                        return Ok(Signal::FAIL(f_obj.status as usize, f_obj.message));
                    } else {
                        return Ok(Signal::FAIL(
                            500,
                            String::from("Error: Unexpected problem occurred"),
                        ));
                    }
                }
                ConditionAction::BREAK => return Ok(Signal::BREAK),
                ConditionAction::CONTINUE => return Ok(Signal::CONTINUE),
            }
        }
    }

    Ok(Signal::NONE)
}
