use crate::{
    components::{
        collection::Collection,
        data::Data,
        datapair::DataPair,
        encryption::EncryptionKey,
        raw_pair::{CustomStructurePair, RawPair, StructurePair},
        structures::Structure,
    },
    utils::{
        data::auto_fetch_all_data, mapping::auto_fetch_all_mappings, validate_stype::validate_stype,
    },
};
use regex::Regex;

pub fn rawpair_to_data(
    all_data: &mut Vec<Data>,
    collection: &Collection,
    raw_pair: &RawPair,
    updating: bool,
) -> Result<String, (usize, String)> {
    let structure_pairs: Vec<StructurePair> = raw_pair.structures.clone();
    let custom_structure_pairs: Vec<CustomStructurePair> = raw_pair.custom_structures.clone();

    let mut all_pairs = Vec::<DataPair>::new();

    if let Err(e) = process_structures(
        &mut all_pairs,
        &structure_pairs,
        &collection.structures,
        "",
        &collection.project_id,
        &collection.id,
        updating,
    ) {
        return Err(e);
    }

    for custom_structure in &collection.custom_structures {
        let custom_structure_id = custom_structure.id.clone();
        let mut target_custom_structure_pair = CustomStructurePair {
            id: custom_structure_id.clone(),
            structures: Vec::<StructurePair>::new(),
        };

        for custom_structure_pair in &custom_structure_pairs {
            if custom_structure_pair.id == custom_structure_id {
                target_custom_structure_pair = custom_structure_pair.clone();
                break;
            }
        }

        if let Err(e) = process_structures(
            &mut all_pairs,
            &target_custom_structure_pair.structures,
            &custom_structure.structures,
            &custom_structure_id,
            &collection.project_id,
            &collection.id,
            updating,
        ) {
            return Err(e);
        }
    }

    let data_id = if raw_pair.data_id.trim().len() > 0 {
        raw_pair.data_id.clone()
    } else {
        EncryptionKey::generate_uuid(16)
    };
    match Data::create(
        all_data,
        &data_id,
        &collection.project_id,
        &collection.id,
        raw_pair.published,
    ) {
        Err(e) => {
            return Err(e);
        }
        Ok(()) => {}
    }

    for pair in all_pairs {
        if let Err(e) = Data::add_pair(all_data, &data_id, pair) {
            println!("Error while adding Datapair: {} ({})", e.1, e.0);
            if let Err(e) = Data::delete(all_data, &data_id) {
                println!(
                    "Error while deleting corrupt data {}: {} ({})",
                    data_id, e.1, e.0
                );
            }
            return Err(e);
        }
    }

    Ok(data_id)
}

fn process_structures(
    all_pairs: &mut Vec<DataPair>,
    structure_pairs: &Vec<StructurePair>,
    structures: &Vec<Structure>,
    custom_structure_id: &str,
    project_id: &str,
    collection_id: &str,
    updating: bool,
) -> Result<(), (usize, String)> {
    for structure in structures {
        let structure_id = structure.id.clone();
        let mut value = String::new();
        let mut used_default = false;

        for structure_pair in structure_pairs {
            if structure_pair.id == structure_id {
                // println!("chosen structure pair: {:#?}", structure_pair);
                value = structure_pair.value.clone();
                break;
            }
        }

        let mut actual_data = Vec::<String>::new();

        let min = structure.min.clone();
        let max = structure.max.clone();
        let stype = structure.stype.clone();
        let unique = structure.unique.clone();
        let required = structure.required.clone();
        let regex_pattern = structure.regex_pattern.clone();
        let array = structure.array.clone();

        if value.len() <= 0 {
            value = structure.default_val.clone();
            used_default = true;
        }

        let mut broken_data: Vec<&str> = vec![&value];
        if array {
            broken_data = value.split(",").collect::<Vec<&str>>();
        }

        for d in broken_data {
            if d.trim().len() > 0 {
                actual_data.push(d.trim().to_string());
            }
        }

        let pair_id = EncryptionKey::generate_uuid(16);
        let processed_dtype = Structure::from_stype(stype.clone());
        let final_data = actual_data.join(",");

        if final_data.len() <= 0 && required {
            return Err((
                400,
                format!("Error: Value is required for structure '{}'", structure_id),
            ));
        }

        for v in actual_data {
            if used_default {
                continue;
            }

            if v.len() < min {
                return Err((
                    400,
                    format!(
                        "Error: Value '{}' is too short for structure '{}'",
                        v, structure_id
                    ),
                ));
            }

            if v.len() > max {
                return Err((
                    400,
                    format!(
                        "Error: Value '{}' is too long for structure '{}'",
                        v, structure_id
                    ),
                ));
            }

            if regex_pattern.len() > 1 {
                if let Ok(re) = Regex::new(&format!(r"{}", regex_pattern)) {
                    if !re.is_match(&v) {
                        return Err((
                            400,
                            format!("Error: Value '{}' does not match regex pattern", v),
                        ));
                    }
                }
            }

            if let Err(e) = validate_stype(&v, stype.clone(), !required) {
                return Err(e);
            }
        }

        let mut found = false;
        let mut count = 0;
        let all_mappings = auto_fetch_all_mappings();
        let all_data = match auto_fetch_all_data(&all_mappings, project_id, collection_id) {
            Ok(data) => data,
            Err(_) => {
                return Err((500, format!("Error: Failed fetching data")));
            }
        };

        if unique && final_data.len() > 0 && !used_default {
            for d in all_data {
                for pair in d.pairs.iter() {
                    if pair.structure_id == structure_id && pair.value == final_data {
                        if count > 0 || !updating {
                            found = true;
                            break;
                        }

                        count += 1;
                    }
                }
            }
        }

        if found {
            return Err((
                400,
                format!("Error: Value '{}' should be unique", final_data),
            ));
        }

        all_pairs.push(DataPair {
            id: pair_id,
            structure_id: structure.id.to_string(),
            custom_structure_id: custom_structure_id.to_string(),
            value: final_data,
            dtype: processed_dtype,
        });
    }

    Ok(())
}
