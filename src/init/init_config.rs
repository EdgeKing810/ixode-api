use crate::{
    components::{
        config::{fetch_all_configs, save_all_configs, Config},
        mapping::{get_file_name, Mapping},
    },
    utils::encryption_key::get_encryption_key,
};

pub fn initialize_configs(mappings: &Vec<Mapping>) -> Vec<Config> {
    let all_configs_path = get_file_name("configs", mappings);
    let mut all_configs = Vec::<Config>::new();

    if let Err(e) = all_configs_path {
        println!("{}", e);
        return all_configs;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_configs = fetch_all_configs(
        all_configs_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    let config_keys_template: Vec<&str> = vec![
        "ENV",
        "PROJECT_NAME",
        "FRONT_URL",
        "API_URL",
        "API_PORT",
        "API_PRE",
        "WS_PORT",
        "MONGO_URI",
        "DB_NAME",
        "JWT_EXPIRE",
        "SMTP_USERNAME",
        "SMTP_PASSWORD",
        "SMTP_HOST",
        "SMTP_PORT",
        "TOKEN_KEY",
        "ENCRYPT_KEY",
        "MISC_KEY",
        "USE_REDIS",
        "REDIS_HOST",
        "REDIS_PORT",
        "UPLOAD_SIZE",
        "SHOULD_INITIALIZE",
        "CORS_WHITELIST",
    ];

    for key in config_keys_template {
        if !Config::exist(&all_configs, key) {
            let create_config = Config::create(&mut all_configs, key, "_empty");
            if let Err(e) = create_config {
                println!("{}", e.1);
            }
        }
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    save_all_configs(
        &all_configs,
        all_configs_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_configs
}
