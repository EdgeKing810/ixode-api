use crate::{
    components::routing::{
        blocks::template_block::TemplateBlock, submodules::sub_ref_data::RefData,
    },
    routes::x_utils::{
        convertors::convert_definition_to_string::definition_to_string,
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::{resolve_conditions, resolve_ref_data},
    },
};

pub fn define_template(
    template_block: TemplateBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
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

    if !res_condition {
        return Ok(DefinitionData::NULL);
    }
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
            match resolve_ref_data(&current_data, global_blocks, all_definitions, current_index) {
                Ok(d) => d,
                Err(e) => {
                    return Err(e);
                }
            }
        };

        match definition_to_string(current_value, str, "Template", false) {
            Ok(s) => {
                final_string = format!("{}{}", final_string, s);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    Ok(DefinitionData::STRING(final_string))
}
