use crate::{
    components::{
        collection::Collection,
        data::Data,
        datapair::DataPair,
        encryption::EncryptionKey,
        structures::{Structure, Type},
    },
    utils::{auto_fetch_all_data, auto_fetch_all_mappings, get_config_value},
};
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RawPair {
    pub structures: Vec<StructurePair>,
    pub custom_structures: Vec<CustomStructurePair>,
    pub published: bool,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StructurePair {
    pub id: String,
    pub value: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomStructurePair {
    pub id: String,
    pub structures: Vec<StructurePair>,
}

pub fn convert_from_raw(
    all_data: &mut Vec<Data>,
    collection: &Collection,
    raw_pair: &RawPair,
    updating: bool,
) -> Result<String, (usize, String)> {
    // println!("{:#?}", raw_pair);

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

    let data_id = EncryptionKey::generate_uuid(16);
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

        // println!("structure id: {}", structure_id);

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

        // println!("value: {}", value);

        let mut broken_data: Vec<&str> = vec![&value];
        if array {
            broken_data = value.split(",").collect::<Vec<&str>>();
        }

        // println!("broken_data: {:#?}", broken_data);

        for d in broken_data {
            if d.trim().len() > 0 {
                actual_data.push(d.trim().to_string());
            }
        }

        // println!("actual_data: {:#?}", actual_data);

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

            if let Err(e) = stype_validator(&v, stype.clone(), !required) {
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

        // println!("Success!");

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

        // println!("structure_id: {}", structure_id);

        for pair in all_pairs {
            if pair.structure_id == structure_id {
                value = pair.value.clone();
                // println!("val!: {}", value);
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
    } else if stype == Type::INTEGER {
        if let Err(_) = data.parse::<isize>() {
            return Err((400, format!("Error: Value '{}' is an invalid integer", data)));
        }
    } else if stype == Type::FLOAT {
        if let Err(_) = data.parse::<f64>() {
            return Err((400, format!("Error: Value '{}' is an invalid float", data)));
        }
    } else if stype == Type::DATE {
        let date_regex = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap();

        if !date_regex.is_match(data) {
            return Err((400, format!("Error: Value '{}' is an invalid date", data)));
        }
    } else if stype == Type::DATETIME {
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
    }else if stype == Type::MEDIA {
        let mappings = auto_fetch_all_mappings();
        let api_url = get_config_value(&mappings, "API_URL", "none").to_lowercase();

        if api_url != String::from("none") && !data.trim().to_lowercase().starts_with(&api_url) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid media url", data),
            ));
        }
    } else if stype == Type::UID {
        let uid_regex = Regex::new(r"^(?:[a-zA-Z0-9]{1,20}-){3}[a-zA-Z0-9]{1,20}$").unwrap();

        if !uid_regex.is_match(data) {
            return Err((400, format!("Error: Value '{}' is an invalid uid", data)));
        }
    } else if stype == Type::JSON {
        if let Err(_) = serde_json::from_str::<serde_json::Value>(&data) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid json object", data),
            ));
        }
    }

    Ok(String::from("Validation OK!"))
}
