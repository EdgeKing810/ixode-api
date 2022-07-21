use crate::{
    components::{
        collection::Collection,
        data::Data,
        datapair::DataPair,
        encryption::EncryptionKey,
        structures::{Structure, Type},
    },
    utils::{auto_fetch_all_mappings, get_config_value},
};
use regex::Regex;
use rocket::serde::{json::from_str, Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RawPair {
    pub structures: Vec<StructurePair>,
    pub custom_structures: Vec<CustomStructurePair>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct StructurePair {
    pub id: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CustomStructurePair {
    pub id: String,
    pub structures: Vec<StructurePair>,
}

// TODO: Updating all Data when Structures/C.S are updated in their functions
// Concerns 'stype' and 'array' only

// TODO: Do necessary actions when structures' ID Change or they get deleted
// or when C.S' ID are changed or they get deleted
// NECESSARY HAS ALREADY BEEN DONE FOR WHEN PROJECTS OR COLLECTIONS GET DELETED

pub fn convert_from_raw(
    all_data: &mut Vec<Data>,
    collection: &Collection,
    raw_pairs: &RawPair,
) -> Result<(), (usize, String)> {
    let structure_pairs: Vec<StructurePair> = raw_pairs.structures.clone();
    let custom_structure_pairs: Vec<CustomStructurePair> = raw_pairs.custom_structures.clone();

    let mut all_pairs = Vec::<DataPair>::new();

    if let Err(e) = process_structures(&mut all_pairs, &structure_pairs, &collection.structures, "")
    {
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
        ) {
            return Err(e);
        }
    }

    let data_id = EncryptionKey::generate_uuid(16);
    match Data::create(all_data, &data_id, &collection.project_id, &collection.id) {
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

    Ok(())
}

fn process_structures(
    all_pairs: &mut Vec<DataPair>,
    structure_pairs: &Vec<StructurePair>,
    structures: &Vec<Structure>,
    custom_structure_id: &str,
) -> Result<(), (usize, String)> {
    for structure in structures {
        let structure_id = structure.id.clone();
        let mut value = String::new();
        let mut used_default = false;

        for structure_pair in structure_pairs {
            if structure_pair.id == structure_id {
                value = structure_pair.value.clone();
                break;
            }
        }

        let mut actual_data = Vec::<String>::new();

        let min = structure.min.clone();
        let max = structure.max.clone();
        let stype = structure.stype.clone();
        let unique = structure.unique.clone();
        // let required = structure.required.clone(); // TODO
        let required = false;
        let regex_pattern = structure.regex_pattern.clone();
        let array = structure.array.clone();

        if value.len() <= 0 && required {
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

        if array {
        } else {
            actual_data.push(value.clone().trim().to_string());
        }

        let pair_id = EncryptionKey::generate_uuid(16);
        let processed_dtype = Structure::from_stype(stype.clone());
        let final_data = actual_data.join(",");

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

            if let Err(e) = stype_validator(&v, stype.clone(), !required) {
                return Err(e);
            }
        }

        let mut found = false;
        if unique && final_data.len() > 0 {
            for pair in all_pairs.iter() {
                if pair.structure_id == structure_id && pair.value == final_data {
                    found = true;
                    break;
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

pub fn convert_to_raw(data: &Data, collection: &Collection) -> Result<RawPair, (usize, String)> {
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
        structures: structure_pairs,
        custom_structures: custom_structure_pairs,
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

        for pair in all_pairs {
            if pair.id == structure_id {
                value = pair.value.clone();
                break;
            }
        }

        if value.len() <= 0 {
            value = structure.default_val.clone();
        }

        let new_structure_pair = StructurePair {
            id: structure_id.clone(),
            value: value.clone(),
        };

        structure_pairs.push(new_structure_pair);
    }
}

pub fn stype_validator(
    data: &str,
    stype: Type,
    is_default: bool,
) -> Result<String, (usize, String)> {
    if is_default && data.len() <= 0 {
        return Ok(String::from("Empty String. Validation Passed!"));
    }

    if stype == Type::EMAIL {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        if !email_regex.is_match(data) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid email address", data),
            ));
        }
    }

    if stype == Type::NUMBER {
        if let Err(_) = data.parse::<f64>() {
            return Err((400, format!("Error: Value '{}' is an invalid number", data)));
        }
    }

    if stype == Type::DATE {
        let datetime_regex = Regex::new(
            r"^[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} (\+|\-)[0-9]{2}:[0-9]{2}$",
        )
        .unwrap();

        if !datetime_regex.is_match(data) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid datetime", data),
            ));
        }
    }

    if stype == Type::MEDIA {
        let mappings = auto_fetch_all_mappings();
        let api_url = get_config_value(&mappings, "API_URL", "none").to_lowercase();

        if api_url != String::from("none") && !data.trim().to_lowercase().starts_with(&api_url) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid media url", data),
            ));
        }
    }

    if stype == Type::UID {
        let uid_regex = Regex::new(r"^(?:[a-zA-Z0-9]{1,20}-){3}[a-zA-Z0-9]{1,20}$").unwrap();

        if !uid_regex.is_match(data) {
            return Err((400, format!("Error: Value '{}' is an invalid uid", data)));
        }
    }

    if stype == Type::JSON {
        let json_data: rocket::serde::json::serde_json::Result<bool> = from_str(&data);
        if let Err(_) = json_data {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid json object", data),
            ));
        }
    }

    Ok(String::from("Validation OK!"))
}
