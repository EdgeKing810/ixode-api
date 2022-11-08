use crate::components::{
    collection::{stringify_collections, Collection},
    event::{stringify_events, Event},
    project::{stringify_projects, Project},
    user::{stringify_users, User},
};

use super::{
    collection::auto_fetch_all_collections, config::get_config_value, event::auto_fetch_all_events,
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

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

    let events = match auto_fetch_all_events(&mappings) {
        Ok(e) => e,
        _ => Vec::<Event>::new(),
    };

    let stringified_events = stringify_events(&events);
    redis::cmd("SET")
        .arg("events")
        .arg(stringified_events)
        .execute(&mut connection);

    String::from("Redis Connection Successful!")
}
