use crate::components::{
    event::{fetch_all_events, save_all_events, stringify_events, unwrap_events, Event},
    mapping::{get_file_name, Mapping},
};

use super::{encryption_key::get_encryption_key, redis::get_redis_connection};

pub fn auto_fetch_all_events(mappings: &Vec<Mapping>) -> Result<Vec<Event>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_events = match redis::pipe().cmd("GET").arg("events").query(&mut con) {
            Ok(e) => Some(e),
            _ => None,
        };

        if let Some(se) = stringified_events {
            return Ok(unwrap_events(se));
        }
    }

    let all_events_path = match get_file_name("events", mappings) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let tmp_password = match std::env::var("TMP_PASSWORD") {
        Ok(pass) => pass,
        _ => "password".to_string(),
    };

    let all_events = fetch_all_events(
        all_events_path.clone(),
        &get_encryption_key(&mappings, &tmp_password),
    );

    Ok(all_events)
}

pub fn auto_save_all_events(mappings: &Vec<Mapping>, events: &Vec<Event>) -> Result<(), String> {
    let all_events_path = match get_file_name("events", mappings) {
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
            .arg("events")
            .arg(stringify_events(events))
            .execute(&mut con);
    }

    save_all_events(events, all_events_path, &encryption_key);

    Ok(())
}

pub fn auto_create_event(
    mappings: &Vec<Mapping>,
    event_type: &str,
    description: String,
    redirect: String,
) -> Result<(), (usize, String)> {
    let mut all_events = match auto_fetch_all_events(mappings) {
        Ok(events) => events,
        Err(e) => {
            println!("{}", e);
            return Err((500, "Error: Failed fetching events".to_string()));
        }
    };

    if let Err(e) = Event::create(&mut all_events, event_type, &description, &redirect) {
        return Err(e);
    }

    if let Err(e) = auto_save_all_events(mappings, &all_events) {
        println!("{}", e);
        return Err((500, "Error: Failed to save events".to_string()));
    }

    Ok(())
}
