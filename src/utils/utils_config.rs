use crate::components::{
    config::{fetch_all_configs, save_all_configs, Config},
    mapping::{get_file_name, Mapping},
};

use super::encryption_key::get_encryption_key;

pub fn auto_fetch_all_configs(mappings: &Vec<Mapping>) -> Result<Vec<Config>, String> {
    let all_configs_path = get_file_name("configs", mappings);

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    if let Err(e) = all_configs_path {
        return Err(e);
    }

    let all_configs = fetch_all_configs(
        all_configs_path.clone().unwrap(),
        &&get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_configs)
}

pub fn auto_save_all_configs(mappings: &Vec<Mapping>, configs: &Vec<Config>) -> Result<(), String> {
    let all_configs_path = match get_file_name("configs", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);
    save_all_configs(configs, all_configs_path, &encryption_key);

    Ok(())
}

pub fn get_config_value(mappings: &Vec<Mapping>, id: &str, default: &str) -> String {
    let all_configs = match auto_fetch_all_configs(mappings) {
        Ok(configs) => configs,
        _ => return default.to_string(),
    };

    if !Config::exist(&all_configs, id) {
        return default.to_string();
    }

    let val = Config::get_value(&all_configs, id);

    if val == "_empty".to_string() {
        return default.to_string();
    }

    val
}

pub fn set_config_value(
    all_configs: &mut Vec<Config>,
    id: &str,
    value: &str,
) -> Result<(), (usize, String)> {
    if !Config::exist(&all_configs, id) {
        return Err((404, "Error: Config does not exist".to_string()));
    }

    match Config::update_value(all_configs, id, value) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}
