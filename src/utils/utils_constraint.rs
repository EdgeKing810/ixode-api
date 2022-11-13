use crate::components::{
    constraint::{
        fetch_all_constraints, save_all_constraints, stringify_constraints, unwrap_constraints,
        Constraint,
    },
    mappings::{get_file_name, Mapping},
};

use super::{encryption_key::get_encryption_key, redis::get_redis_connection};

pub fn auto_fetch_all_constraints(mappings: &Vec<Mapping>) -> Result<Vec<Constraint>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_constraints =
            match redis::pipe().cmd("GET").arg("constraints").query(&mut con) {
                Ok(c) => Some(c),
                _ => None,
            };

        if let Some(sc) = stringified_constraints {
            match unwrap_constraints(sc) {
                Ok(sc) => return Ok(sc),
                Err(e) => return Err(e.1),
            }
        }
    }

    let all_constraints_path = match get_file_name("constraints", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    match fetch_all_constraints(
        all_constraints_path.clone(),
        &get_encryption_key(&mappings, &tmp_password),
    ) {
        Ok(constraints) => Ok(constraints),
        Err(e) => Err(e.1),
    }
}

pub fn auto_save_all_constraints(
    mappings: &Vec<Mapping>,
    constraints: &Vec<Constraint>,
) -> Result<(), String> {
    let all_constraints_path = match get_file_name("constraints", mappings) {
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
            .arg("constraints")
            .arg(stringify_constraints(constraints))
            .execute(&mut con);
    }

    save_all_constraints(constraints, all_constraints_path, &encryption_key);

    Ok(())
}
