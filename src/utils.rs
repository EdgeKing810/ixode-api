use crate::components::collection::{
    fetch_all_collections, save_all_collections, stringify_collections, unwrap_collections,
    Collection,
};
use crate::components::config::{fetch_all_configs, save_all_configs, Config};
use crate::components::io::{fetch_file, remove_file, save_file};
use crate::components::mappings::{fetch_all_mappings, get_file_name, save_all_mappings, Mapping};
use crate::components::project::{
    fetch_all_projects, save_all_projects, stringify_projects, unwrap_projects, Project,
};
use crate::components::user::{
    fetch_all_users, save_all_users, stringify_users, unwrap_users, User,
};
use crate::init::initialize_encryption_key;

pub fn get_encryption_key(all_mappings: &Vec<Mapping>, tmp_password: &str) -> String {
    let init_encryption = initialize_encryption_key(&all_mappings, tmp_password);

    if let Err(e) = init_encryption {
        println!("Error: {}", e);
        return String::new();
    }

    init_encryption.unwrap()
}

pub fn get_redis_connection() -> Result<redis::Connection, String> {
    let mappings = auto_fetch_all_mappings();

    let use_redis = get_config_value(&mappings, "USE_REDIS", "y");
    let host = get_config_value(&mappings, "REDIS_HOST", "");
    // let port = get_config_value(&mappings, "REDIS_PORT", "6379");
    // let db = get_config_value(&mappings, "REDIS_DB_NAME", "kinesis-api");

    if use_redis.to_lowercase() != "y" && use_redis.to_lowercase() != "yes" {
        return Err(String::from("Error: USE_REDIS not set to yes"));
    }

    if host.trim().len() <= 1 {
        return Err(String::from("Error: REDIS_HOST not set"));
    }

    let client = match redis::Client::open(format!("redis://{}/", host)) {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            // println!("{}", format!("redis://{}:{}/{}", host, port, db));
            return Err(String::from("Error: Redis Connection failed (Invalid URI)"));
        }
    };

    let conn = match client.get_connection() {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return Err(String::from("Error: Redis Connection failed"));
        }
    };

    Ok(conn)
}

pub fn has_redis_connection() -> bool {
    match get_redis_connection() {
        Ok(_) => true,
        _ => false,
    }
}

pub fn init_redis() -> String {
    let mappings = auto_fetch_all_mappings();

    let mut connection = match get_redis_connection() {
        Ok(conn) => conn,
        Err(e) => {
            return e;
        }
    };

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => Vec::<User>::new(),
    };

    let stringified_users = stringify_users(&users);
    redis::cmd("SET")
        .arg("users")
        .arg(stringified_users)
        .execute(&mut connection);

    let projects = match auto_fetch_all_projects(&mappings) {
        Ok(p) => p,
        _ => Vec::<Project>::new(),
    };

    let stringified_projects = stringify_projects(&projects);
    redis::cmd("SET")
        .arg("projects")
        .arg(stringified_projects)
        .execute(&mut connection);

    let collections = match auto_fetch_all_collections(&mappings) {
        Ok(c) => c,
        _ => Vec::<Collection>::new(),
    };

    let stringified_collections = stringify_collections(&collections);
    redis::cmd("SET")
        .arg("collections")
        .arg(stringified_collections)
        .execute(&mut connection);

    String::from("Redis Connection Successful!")
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

pub fn auto_fetch_all_users(mappings: &Vec<Mapping>) -> Result<Vec<User>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_users = match redis::pipe().cmd("GET").arg("users").query(&mut con) {
            Ok(u) => Some(u),
            _ => None,
        };

        if let Some(su) = stringified_users {
            return Ok(unwrap_users(su));
        }
    }

    let all_users_path = match get_file_name("users", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let all_users = fetch_all_users(
        all_users_path.clone(),
        &get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_users)
}

pub fn auto_save_all_users(mappings: &Vec<Mapping>, users: &Vec<User>) -> Result<(), String> {
    let all_users_path = match get_file_name("users", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);

    let connection = get_redis_connection();
    if let Ok(mut con) = connection {
        redis::cmd("SET")
            .arg("users")
            .arg(stringify_users(users))
            .execute(&mut con);
    }

    save_all_users(users, all_users_path, &encryption_key);

    Ok(())
}

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
        &get_encryption_key(&mappings, &tmp_password),
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

pub fn auto_fetch_all_projects(mappings: &Vec<Mapping>) -> Result<Vec<Project>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_projects = match redis::pipe().cmd("GET").arg("projects").query(&mut con) {
            Ok(p) => Some(p),
            _ => None,
        };

        if let Some(sp) = stringified_projects {
            return Ok(unwrap_projects(sp));
        }
    }

    let all_projects_path = match get_file_name("projects", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let all_projects = fetch_all_projects(
        all_projects_path.clone(),
        &get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_projects)
}

pub fn auto_save_all_projects(
    mappings: &Vec<Mapping>,
    projects: &Vec<Project>,
) -> Result<(), String> {
    let all_projects_path = match get_file_name("projects", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);

    let connection = get_redis_connection();
    if let Ok(mut con) = connection {
        redis::cmd("SET")
            .arg("projects")
            .arg(stringify_projects(projects))
            .execute(&mut con);
    }

    save_all_projects(projects, all_projects_path, &encryption_key);

    Ok(())
}

pub fn auto_fetch_all_collections(mappings: &Vec<Mapping>) -> Result<Vec<Collection>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_collections =
            match redis::pipe().cmd("GET").arg("collections").query(&mut con) {
                Ok(c) => Some(c),
                _ => None,
            };

        if let Some(sc) = stringified_collections {
            return Ok(unwrap_collections(sc));
        }
    }

    let all_collections_path = match get_file_name("collections", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let all_collections = fetch_all_collections(
        all_collections_path.clone(),
        &get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_collections)
}

pub fn auto_save_all_collections(
    mappings: &Vec<Mapping>,
    collections: &Vec<Collection>,
) -> Result<(), String> {
    let all_collections_path = match get_file_name("collections", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);

    let connection = get_redis_connection();
    if let Ok(mut con) = connection {
        redis::cmd("SET")
            .arg("collections")
            .arg(stringify_collections(collections))
            .execute(&mut con);
    }

    save_all_collections(collections, all_collections_path, &encryption_key);

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
