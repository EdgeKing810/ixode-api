use crate::io::{fetch_file, save_file};

#[derive(Default, Debug, Clone)]
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

    pub fn create(all_configs: &mut Vec<Config>, name: &str, value: &str) -> Result<(), String> {
        if !String::from(name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
        {
            return Err(String::from("Error: name contains an invalid character"));
        }

        if String::from(name.trim()).len() < 1 {
            return Err(String::from(
                "Error: name does not contain enough characters",
            ));
        } else if String::from(name.trim()).len() > 100 {
            return Err(String::from("Error: name contains too many characters"));
        }

        for config in all_configs.iter_mut() {
            if config.name.to_lowercase() == name.to_string().to_lowercase() {
                return Err(format!(
                    "Error: A config with that name already exists ({})",
                    config.name
                ));
            }
        }

        let new_config = Config {
            name: name.trim().to_string(),
            value: value.trim().to_string(),
        };
        all_configs.push(new_config);

        let update_value_result = Self::update_value(all_configs, name, value);
        if let Err(e) = update_value_result {
            let delete_result = Self::delete(all_configs, name);
            if let Err(e) = delete_result {
                return Err(e);
            }
            return Err(e);
        }

        Ok(())
    }

    pub fn update_value(
        all_configs: &mut Vec<Config>,
        name: &str,
        value: &str,
    ) -> Result<(), String> {
        let mut found_config: Option<Config> = None;

        if String::from(value).chars().any(|c| c == '|') {
            return Err(String::from("Error: value contains an invalid character"));
        }

        if String::from(value.trim()).len() < 1 {
            return Err(String::from(
                "Error: value does not contain enough characters",
            ));
        } else if String::from(value.trim()).len() > 200 {
            return Err(String::from("Error: value contains too many characters"));
        }

        for config in all_configs.iter_mut() {
            if config.name == name.to_string() {
                found_config = Some(config.clone());
                config.value = value.trim().to_string();
                break;
            }
        }

        if let None = found_config {
            return Err(String::from("Error: Config not found"));
        }

        Ok(())
    }

    pub fn delete(all_configs: &mut Vec<Config>, name: &str) -> Result<(), String> {
        let mut found_config: Option<Config> = None;

        for config in all_configs.iter_mut() {
            if config.name == name.to_string() {
                found_config = Some(config.clone());
                break;
            }
        }

        if let None = found_config {
            return Err(String::from("Error: Config not found"));
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
}

pub fn fetch_all_configs(path: String) -> Vec<Config> {
    let all_configs_raw = fetch_file(path.clone());

    let individual_configs = all_configs_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_configs: Vec<Config> = Vec::<Config>::new();

    for config in individual_configs {
        let current_config = config.split("|").collect::<Vec<&str>>();

        let tmp_config = Config::create_no_check(current_config[0], current_config[1]);
        final_configs.push(tmp_config);
    }

    final_configs
}

pub fn save_all_configs(configs: &Vec<Config>, path: &str) {
    let mut stringified_configs = String::new();

    for config in configs {
        stringified_configs = format!(
            "{}{}{}|{}",
            stringified_configs,
            if stringified_configs.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            config.name,
            config.value,
        );
    }

    save_file(String::from(path), stringified_configs);
    println!("Configs saved!");
}