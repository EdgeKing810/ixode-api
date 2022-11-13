use crate::components::{
    mapping::{get_file_name, Mapping},
    media::{fetch_all_medias, save_all_medias, stringify_medias, unwrap_medias, Media},
};

use super::{encryption_key::get_encryption_key, redis::get_redis_connection};

pub fn auto_fetch_all_medias(mappings: &Vec<Mapping>) -> Result<Vec<Media>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_medias = match redis::pipe().cmd("GET").arg("medias").query(&mut con) {
            Ok(m) => Some(m),
            _ => None,
        };

        if let Some(sm) = stringified_medias {
            return Ok(unwrap_medias(sm));
        }
    }

    let all_medias_path = match get_file_name("medias", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let all_medias = fetch_all_medias(
        all_medias_path.clone(),
        &get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_medias)
}

pub fn auto_save_all_medias(mappings: &Vec<Mapping>, medias: &Vec<Media>) -> Result<(), String> {
    let all_medias_path = match get_file_name("medias", mappings) {
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
            .arg("medias")
            .arg(stringify_medias(medias))
            .execute(&mut con);
    }

    save_all_medias(medias, all_medias_path, &encryption_key);

    Ok(())
}
