use crate::{
    components::routing::blocks::loop_block::LoopBlock,
    routes::x_utils::{
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::resolve_ref_data,
    },
};

pub fn define_loop(
    loop_block: LoopBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
    resolve_ref_data(
        &loop_block.min,
        global_blocks,
        all_definitions,
        current_index,
    )
}
