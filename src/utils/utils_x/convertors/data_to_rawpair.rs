use crate::components::{
    collection::Collection,
    data::Data,
    datapair::DataPair,
    raw_pair::{CustomStructurePair, RawPair, StructurePair},
    routing::submodules::sub_body_data_type::BodyDataType,
    structure::{Structure, Type},
};

pub fn data_to_rawpair(data: &Data, collection: &Collection) -> Result<RawPair, (usize, String)> {
    let mut structure_pairs = Vec::<StructurePair>::new();
    let mut custom_structure_pairs = Vec::<CustomStructurePair>::new();

    let filtered_data_pairs = data
        .pairs
        .clone()
        .iter_mut()
        .filter(|pair| pair.custom_structure_id == "")
        .map(|pair| DataPair {
            id: pair.id.clone(),
            structure_id: pair.structure_id.to_string(),
            custom_structure_id: "".to_string(),
            value: pair.value.clone(),
            dtype: pair.dtype.clone(),
        })
        .collect::<Vec<DataPair>>();

    revert_structures(
        &filtered_data_pairs,
        &mut structure_pairs,
        &collection.structures,
    );

    for custom_structure in &collection.custom_structures {
        let custom_structure_id = custom_structure.id.clone();
        let filtered_data_pairs = data
            .pairs
            .clone()
            .iter_mut()
            .filter(|pair| pair.custom_structure_id == custom_structure_id)
            .map(|pair| DataPair {
                id: pair.id.clone(),
                structure_id: pair.structure_id.clone(),
                custom_structure_id: custom_structure_id.clone(),
                value: pair.value.clone(),
                dtype: pair.dtype.clone(),
            })
            .collect::<Vec<DataPair>>();

        let mut temporary_structure_pairs = Vec::<StructurePair>::new();
        revert_structures(
            &filtered_data_pairs,
            &mut temporary_structure_pairs,
            &custom_structure.structures,
        );

        let new_custom_structure_pair = CustomStructurePair {
            id: custom_structure_id.clone(),
            structures: temporary_structure_pairs,
        };
        custom_structure_pairs.push(new_custom_structure_pair);
    }

    let raw_pair = RawPair {
        data_id: data.id.clone(),
        structures: structure_pairs,
        custom_structures: custom_structure_pairs,
        published: data.published,
    };

    Ok(raw_pair)
}

fn revert_structures(
    all_pairs: &Vec<DataPair>,
    structure_pairs: &mut Vec<StructurePair>,
    structures: &Vec<Structure>,
) {
    for structure in structures {
        let structure_id = structure.id.clone();
        let mut value = String::new();

        let mut rtype = BodyDataType::STRING;

        for pair in all_pairs {
            if pair.structure_id == structure_id {
                value = pair.value.clone();

                if structure.array {
                    rtype = BodyDataType::ARRAY;
                } else {
                    rtype = match structure.stype {
                        Type::BOOLEAN => BodyDataType::BOOLEAN,
                        Type::INTEGER => BodyDataType::INTEGER,
                        Type::FLOAT => BodyDataType::FLOAT,
                        Type::ENUM => BodyDataType::ARRAY,
                        Type::CUSTOM(_) => BodyDataType::OTHER,
                        _ => BodyDataType::STRING,
                    }
                }

                break;
            }
        }

        if value.len() <= 0 {
            value = structure.default_val.clone();
        }

        let new_structure_pair = StructurePair {
            id: structure_id.clone(),
            value: value.clone(),
            rtype: BodyDataType::to(rtype),
        };

        structure_pairs.push(new_structure_pair);
    }
}
