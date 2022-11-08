use crate::components::mappings::{fetch_all_mappings, save_all_mappings, Mapping};

use super::encryption_key::get_encryption_key;

pub fn auto_fetch_all_mappings() -> Vec<Mapping> {
    let mappings_path = format!(
        "{}{}",
        match std::env::var("CURRENT_PATH") {
            Ok(path) => path,
            _ => "/tmp".to_string(),
        },
        "/data/mappings.txt"
    );

    fetch_all_mappings(&mappings_path, &String::new())
}

pub fn auto_save_all_mappings(mappings: &Vec<Mapping>) -> Result<(), String> {
    let mappings_path = format!(
        "{}{}",
        match std::env::var("CURRENT_PATH") {
            Ok(path) => path,
            _ => "/tmp".to_string(),
        },
        "/data/mappings.txt"
    );

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);
    save_all_mappings(mappings, &mappings_path, &encryption_key);

    Ok(())
}
