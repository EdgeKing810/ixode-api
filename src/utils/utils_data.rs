use crate::components::{
    data::{fetch_all_data, save_all_data, stringify_data, unwrap_data, Data},
    mapping::Mapping,
};

use super::{
    encryption_key::get_encryption_key, io::get_root_data_dir, redis::get_redis_connection,
};

pub fn auto_fetch_all_data(
    mappings: &Vec<Mapping>,
    project_id: &str,
    collection_id: &str,
) -> Result<Vec<Data>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_data = match redis::pipe()
            .cmd("GET")
            .arg(format!("data_{}_{}", project_id, collection_id))
            .query(&mut con)
        {
            Ok(d) => Some(d),
            _ => None,
        };

        if let Some(sd) = stringified_data {
            return Ok(unwrap_data(sd));
        }
    }

    let all_data_path = format!(
        "{}/data/projects/{}/{}/data.txt",
        get_root_data_dir(),
        project_id,
        collection_id
    );

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let all_data = fetch_all_data(
        all_data_path.clone(),
        &get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_data)
}

pub fn auto_save_all_data(
    mappings: &Vec<Mapping>,
    project_id: &str,
    collection_id: &str,
    data: &Vec<Data>,
) -> Result<(), String> {
    let all_data_path = format!(
        "{}/data/projects/{}/{}/data.txt",
        get_root_data_dir(),
        project_id,
        collection_id
    );

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let encryption_key = get_encryption_key(mappings, &tmp_password);

    let connection = get_redis_connection();
    if let Ok(mut con) = connection {
        redis::cmd("SET")
            .arg(format!("data_{}_{}", project_id, collection_id))
            .arg(stringify_data(data))
            .execute(&mut con);
    }

    save_all_data(data, all_data_path, &encryption_key);

    Ok(())
}
