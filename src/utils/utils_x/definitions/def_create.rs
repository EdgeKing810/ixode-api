use crate::{
    components::{collection::Collection, data::Data, routing::blocks::create_block::CreateBlock},
    utils::x::{
        convertors::convert_rawpair_to_data::rawpair_to_data,
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::{resolve_conditions, resolve_raw_data},
    },
    utils::{
        collection::auto_fetch_all_collections,
        data::{auto_fetch_all_data, auto_save_all_data},
        mapping::auto_fetch_all_mappings,
    },
};

pub fn define_create(
    create_block: CreateBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
    project_id: &str,
) -> Result<DefinitionData, (usize, String)> {
    let res_condition = match resolve_conditions(
        &create_block.conditions,
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

    let return_definition: DefinitionData;

    let mappings = auto_fetch_all_mappings();
    let all_data = match auto_fetch_all_data(&mappings, project_id, &create_block.ref_col) {
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

    let collection = match Collection::get(&all_collections, project_id, &create_block.ref_col) {
        Ok(p) => p,
        Err(_) => {
            return Err((
                404,
                String::from("Error: No Collection with this collection_id found"),
            ));
        }
    };

    let mut current_data = Data::get_all(&all_data, project_id, &create_block.ref_col);

    match resolve_raw_data(
        &create_block.ref_object,
        global_blocks,
        all_definitions,
        current_index,
    ) {
        Ok(def) => match def {
            DefinitionData::DATA(d) => {
                return_definition = DefinitionData::DATA(d.clone());

                match rawpair_to_data(&mut current_data, &collection, &d, true) {
                    Err(e) => {
                        return Err(e);
                    }
                    _ => {}
                }
            }
            _ => {
                return Err((
                    500,
                    format!(
                        "Error: Invalid data type for '{}' in Create",
                        create_block.ref_object
                    ),
                ));
            }
        },
        Err(e) => {
            return Err(e);
        }
    }

    if create_block.save {
        match auto_save_all_data(&mappings, &project_id, &create_block.ref_col, &current_data) {
            Err(e) => {
                return Err((500, e));
            }
            _ => {}
        }
    }

    Ok(return_definition)
}
