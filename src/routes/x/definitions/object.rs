use crate::{
    components::{
        raw_pair::{CustomStructurePair, RawPair, StructurePair},
        routing::{
            blocks::object_block::ObjectBlock, submodules::sub_body_data_type::BodyDataType,
        },
    },
    routes::x_utils::{
        convertors::convert_definition_to_string::definition_to_string,
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::resolve_ref_data,
    },
};

pub fn define_object(
    object_block: ObjectBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
    let mut structures = Vec::<StructurePair>::new();
    let mut custom_structures = Vec::<CustomStructurePair>::new();

    for pair in object_block.pairs {
        let current_data =
            match resolve_ref_data(&pair.data, global_blocks, all_definitions, current_index) {
                Ok(d) => d,
                Err(e) => {
                    return Err(e);
                }
            };

        let mut processed_data = String::new();

        match current_data {
            DefinitionData::DATA(d) => {
                let mut current_structures = Vec::<StructurePair>::new();

                for structure in d.structures {
                    current_structures.push(StructurePair {
                        id: structure.id,
                        value: structure.value,
                        rtype: structure.rtype,
                    });
                }

                for custom_structure in d.custom_structures {
                    for structure in custom_structure.structures {
                        current_structures.push(StructurePair {
                            id: format!("{}.{}", custom_structure.id, structure.id),
                            value: structure.value,
                            rtype: structure.rtype,
                        });
                    }
                }

                custom_structures.push(CustomStructurePair {
                    id: pair.id,
                    structures: current_structures,
                });

                continue;
            }
            DefinitionData::NULL => {}
            DefinitionData::UNDEFINED => {}
            _ => {
                processed_data =
                    definition_to_string(current_data.clone(), "", "Update", false).unwrap();
            }
        }

        structures.push(StructurePair {
            id: pair.id,
            value: processed_data,
            rtype: BodyDataType::to(pair.data.rtype),
        });
    }

    Ok(DefinitionData::DATA(RawPair {
        data_id: String::new(),
        structures: structures,
        custom_structures: custom_structures,
        published: false,
    }))
}
