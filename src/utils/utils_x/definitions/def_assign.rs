use crate::{
    components::routing::blocks::assignment_block::AssignmentBlock,
    utils::x::{
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::{resolve_conditions, resolve_operations},
    },
};

pub fn define_assign(
    assign_block: AssignmentBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
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

    if !res_condition {
        return Ok(DefinitionData::NULL);
    }

    resolve_operations(
        &assign_block.operations,
        global_blocks,
        all_definitions,
        current_index,
    )
}
