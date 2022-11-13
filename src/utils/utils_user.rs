use crate::components::{
    mapping::{get_file_name, Mapping},
    user::{fetch_all_users, save_all_users, stringify_users, unwrap_users, User},
};

use super::{encryption_key::get_encryption_key, redis::get_redis_connection};

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
        &&get_encryption_key(&mappings, &tmp_password),
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
