use crate::components::io::remove_file;
use crate::components::mappings::{fetch_all_mappings, get_file_name, Mapping};
use crate::components::user::{fetch_all_users, User};
use crate::init::initialize_encryption_key;

pub fn get_encryption_key(all_mappings: &Vec<Mapping>, tmp_password: &str) -> String {
    let init_encryption = initialize_encryption_key(&all_mappings, tmp_password);

    if let Err(e) = init_encryption {
        println!("Error: {}", e);
        return String::new();
    }

    init_encryption.unwrap()
}

pub fn reset_db(all_mappings: Vec<Mapping>, path: &str) {
    remove_file(path.to_string());
    for mapping in all_mappings.iter() {
        remove_file(mapping.get_file_name());
    }
}

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

pub fn auto_fetch_all_users(mappings: &Vec<Mapping>) -> Result<Vec<User>, String> {
    let all_users_path = get_file_name("users", mappings);

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    if let Err(e) = all_users_path {
        return Err(e);
    }

    let all_users = fetch_all_users(
        all_users_path.clone().unwrap(),
        &get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_users)
}
