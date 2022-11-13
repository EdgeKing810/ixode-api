use crate::components::{
    collection::{
        fetch_all_collections, save_all_collections, stringify_collections, unwrap_collections,
        Collection,
    },
    mapping::{get_file_name, Mapping},
};

use super::{encryption_key::get_encryption_key, redis::get_redis_connection};

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
