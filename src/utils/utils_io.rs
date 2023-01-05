use crate::components::{
    io::{
        ensure_directory_exists, fetch_file, remove_directory, remove_file, rename_directory,
        save_file,
    },
    mapping::Mapping,
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

pub fn auto_save_file_unencrypted(path: &str, data: &str) {
    save_file(path.to_string(), data.to_string(), &String::new());
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

pub fn obtain_lock_name(path: &str) -> String {
    let root_dir = get_root_data_dir();
    let mut lock_name = path.replace(&root_dir, "");

    if lock_name.contains("/") {
        let split: Vec<&str> = lock_name.split("/").collect();
        lock_name = split.join(".");
        let split: Vec<&str> = lock_name.split(".txt").collect();
        lock_name = split.join("");
        let split: Vec<&str> = lock_name.split(".data.").collect();
        lock_name = split.join("");
    }

    lock_name
}

pub fn auto_check_lock(lock_name: &str) -> bool {
    let lock_directory = format!("{}/.lock/", get_root_data_dir());
    ensure_directory_exists(&lock_directory);

    let mut found = false;
    for entry in std::fs::read_dir(&lock_directory).unwrap() {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();

            if let Ok(ft) = entry.file_type() {
                if !ft.is_dir() {
                    if file_name.to_str().unwrap().contains(lock_name) {
                        found = true;
                        break;
                    }
                }
            }
        }
    }
    found
}

pub fn auto_create_lock(lock_name: &str) -> bool {
    if lock_name.trim().len() < 1 {
        return false;
    }

    let lock_file_path = format!("{}/.lock/{}.lock", get_root_data_dir(), lock_name);

    if auto_check_lock(lock_name) {
        return false;
    }

    auto_save_file_unencrypted(&lock_file_path, "");
    true
}

pub fn auto_release_lock(lock_name: &str) -> bool {
    if lock_name.trim().len() < 1 {
        return false;
    }

    let lock_file_path = format!("{}/.lock/{}.lock", get_root_data_dir(), lock_name);

    if !auto_check_lock(lock_name) {
        return false;
    }

    remove_file(lock_file_path);
    true
}

pub fn auto_reset_locks() -> bool {
    let lock_directory = format!("{}/.lock/", get_root_data_dir());
    ensure_directory_exists(&lock_directory);

    for entry in std::fs::read_dir(&lock_directory).unwrap() {
        if let Ok(entry) = entry {
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    remove_directory(&entry.path().to_str().unwrap().to_string());
                } else {
                    remove_file(entry.path().to_str().unwrap().to_string());
                }
            }
        }
    }

    true
}
