use crate::{components::io::{fetch_file, save_file}, utils::{mapping::auto_fetch_all_mappings, constraint::auto_fetch_all_constraints}};
use rocket::serde::{Deserialize, Serialize};

use super::constraint_property::ConstraintProperty;

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub name: String,
    pub value: String,
}

impl Config {
    fn create_no_check(name: &str, value: &str) -> Config {
        Config {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
        }
    }

    pub fn exist(all_configs: &Vec<Config>, name: &str) -> bool {
        let mut found = false;
        for config in all_configs.iter() {
            if config.name.to_lowercase() == name.to_lowercase() {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get_value(all_configs: &Vec<Config>, name: &str) -> String {
        for config in all_configs.iter() {
            if config.name.to_lowercase() == name.to_lowercase() {
                return config.value.clone();
            }
        }

        String::from("")
    }

    pub fn create(
        all_configs: &mut Vec<Config>,
        name: &str,
        value: &str,
    ) -> Result<(), (usize, String)> {
        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_name = match ConstraintProperty::validate(&all_constraints, "config", "name", name) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for config in all_configs.iter_mut() {
            if config.name.to_lowercase() == name.to_string().to_lowercase() {
                return Err((
                    403,
                    format!(
                        "Error: A config with that name already exists ({})",
                        config.name
                    ),
                ));
            }
        }

        let new_config = Config {
            name: final_name,
            value: String::new(),
        };
        all_configs.push(new_config);

        match Self::update_value(all_configs, name, value) {
            Err(e1) => match Self::delete(all_configs, name) {
                Err(e2) => return Err(e2),
                _ => return Err(e1),
            },
            _ => {}
        }

        Ok(())
    }

    pub fn update_value(
        all_configs: &mut Vec<Config>,
        name: &str,
        value: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_config: Option<Config> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(&all_constraints, "config", "value", value) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for config in all_configs.iter_mut() {
            if config.name == name.to_string() {
                found_config = Some(config.clone());
                config.value = final_value;
                break;
            }
        }

        if let None = found_config {
            return Err((404, String::from("Error: Config not found")));
        }

        Ok(())
    }

    pub fn delete(all_configs: &mut Vec<Config>, name: &str) -> Result<(), (usize, String)> {
        let mut found_config: Option<Config> = None;

        for config in all_configs.iter_mut() {
            if config.name == name.to_string() {
                found_config = Some(config.clone());
                break;
            }
        }

        if let None = found_config {
            return Err((404, String::from("Error: Config not found")));
        }

        let updated_configs: Vec<Config> = all_configs
            .iter_mut()
            .filter(|config| config.name != *name)
            .map(|config| Config {
                name: config.name.clone(),
                value: config.value.clone(),
            })
            .collect::<Vec<Config>>();

        *all_configs = updated_configs;

        Ok(())
    }

    pub fn to_string(config: Config) -> String {
        format!("{}|{}", config.name, config.value)
    }

    pub fn from_string(config_str: &str) -> Config {
        let current_config = config_str.split("|").collect::<Vec<&str>>();

        Config::create_no_check(current_config[0], current_config[1])
    }
}

pub fn fetch_all_configs(path: String, encryption_key: &String) -> Vec<Config> {
    let all_configs_raw = fetch_file(path.clone(), encryption_key);

    let individual_configs = all_configs_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_configs: Vec<Config> = Vec::<Config>::new();

    for config in individual_configs {
        let tmp_config = Config::from_string(config);
        final_configs.push(tmp_config);
    }

    final_configs
}

pub fn save_all_configs(configs: &Vec<Config>, path: String, encryption_key: &String) {
    let mut stringified_configs = String::new();

    for config in configs {
        stringified_configs = format!(
            "{}{}{}",
            stringified_configs,
            if stringified_configs.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Config::to_string(config.clone()),
        );
    }

    save_file(path, stringified_configs, encryption_key);
    println!("Configs saved!");
}
