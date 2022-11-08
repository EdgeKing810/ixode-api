use crate::{
    components::{collection::Collection, data::Data, routing::blocks::fetch_block::FetchBlock},
    routes::x_utils::{
        convertors::convert_data_to_rawpair::data_to_rawpair, definition_store::DefinitionData,
    },
    utils::{
        collection::auto_fetch_all_collections, data::auto_fetch_all_data,
        mapping::auto_fetch_all_mappings,
    },
};

pub fn define_fetch(
    fetch_block: FetchBlock,
    project_id: &str,
) -> Result<DefinitionData, (usize, String)> {
    let mappings = auto_fetch_all_mappings();
    let all_data = match auto_fetch_all_data(&mappings, &project_id, &fetch_block.ref_col) {
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

    let collection = match Collection::get(&all_collections, project_id, &fetch_block.ref_col) {
        Ok(p) => p,
        Err(_) => {
            return Err((
                404,
                String::from("Error: No Collection with this collection_id found"),
            ));
        }
    };

    let current_data = Data::get_all(&all_data, project_id, &fetch_block.ref_col);
    let mut all_definitions = Vec::<DefinitionData>::new();

    for data in current_data {
        match data_to_rawpair(&data, &collection) {
            Ok(rp) => {
                all_definitions.push(DefinitionData::DATA(rp));
            }
            Err(e) => {
                return Err(e);
            }
        };
    }

    Ok(DefinitionData::ARRAY(all_definitions))
}
