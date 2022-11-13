use crate::components::{
    mapping::{get_file_name, Mapping},
    project::{
        fetch_all_projects, save_all_projects, stringify_projects, unwrap_projects, Project,
    },
};

use super::{encryption_key::get_encryption_key, redis::get_redis_connection};

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
