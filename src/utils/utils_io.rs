use crate::components::{
    io::{ensure_directory_exists, fetch_file, remove_directory, rename_directory, save_file},
    mappings::Mapping,
};

use super::encryption_key::get_encryption_key;

pub fn auto_fetch_file(path: &str, mappings: &Vec<Mapping>) -> String {
    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);

    fetch_file(path.to_string(), &encryption_key)
}

pub fn auto_save_file(path: &str, data: &str, mappings: &Vec<Mapping>) {
    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);

    save_file(path.to_string(), data.to_string(), &encryption_key);
}

pub fn get_root_data_dir() -> String {
    match std::env::var("CURRENT_PATH") {
        Ok(path) => path,
        _ => "/tmp".to_string(),
    }
}

pub fn auto_create_directory(path: &str) {
    let dir = get_root_data_dir();
    let complete_path = format!("{}{}", dir, path);
    ensure_directory_exists(&complete_path);
}

pub fn auto_rename_directory(old_path: &str, path: &str) {
    let dir = get_root_data_dir();
    let complete_old_path = format!("{}{}", dir, old_path);
    let complete_new_path = format!("{}{}", dir, path);
    rename_directory(&complete_old_path, &complete_new_path);
}

pub fn auto_remove_directory(path: &str) {
    let dir = get_root_data_dir();
    let complete_path = format!("{}{}", dir, path);
    remove_directory(&complete_path);
}
