use crate::components::{
    encryption::EncryptionKey,
    io::{fetch_file, save_file},
};
use chrono::prelude::*;
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Event {
    pub id: String,
    pub event_type: String,
    pub description: String,
    pub timestamp: String,
    pub redirect: String,
}

impl Event {
    fn create_no_check(
        id: &str,
        event_type: &str,
        description: &str,
        timestamp: &str,
        redirect: &str,
    ) -> Event {
        Event {
            id: id.trim().to_string(),
            event_type: event_type.trim().to_string(),
            description: description.trim().to_string(),
            timestamp: timestamp.trim().to_string(),
            redirect: redirect.trim().to_string(),
        }
    }

    pub fn create(
        all_events: &mut Vec<Event>,
        event_type: &str,
        description: &str,
        redirect: &str,
    ) -> Result<(), (usize, String)> {
        let new_id = EncryptionKey::generate_uuid(16);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_event = Event {
            id: new_id.clone(),
            event_type: "".to_string(),
            description: "".to_string(),
            timestamp: "".to_string(),
            redirect: "".to_string(),
        };
        all_events.push(new_event);

        if !has_error {
            let event_type_update = Self::update_event_type(all_events, &new_id, event_type);
            if let Err(e) = event_type_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let description_update = Self::update_description(all_events, &new_id, description);
            if let Err(e) = description_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let timestamp_update = Self::update_timestamp(all_events, &new_id);
            if let Err(e) = timestamp_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let redirect_update = Self::update_redirect(all_events, &new_id, redirect);
            if let Err(e) = redirect_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_event = Self::delete(all_events, &new_id);
            if let Err(e) = delete_event {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_events: &Vec<Event>, id: &str) -> bool {
        let mut found = false;
        for event in all_events.iter() {
            if event.id.to_lowercase() == id.to_lowercase() {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(all_events: &Vec<Event>, event_id: &str) -> Result<Event, (usize, String)> {
        for event in all_events.iter() {
            if event.id.to_lowercase() == event_id.to_lowercase() {
                return Ok(event.clone());
            }
        }

        Err((404, String::from("Error: Event not found")))
    }

    pub fn update_event_type(
        all_events: &mut Vec<Event>,
        id: &String,
        event_type: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_event: Option<Event> = None;

        if !String::from(event_type)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: event_type contains an invalid character"),
            ));
        }

        if String::from(event_type.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: event_type does not contain enough characters"),
            ));
        } else if String::from(event_type.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: event_type contains too many characters"),
            ));
        }

        for event in all_events.iter_mut() {
            if event.id == *id {
                found_event = Some(event.clone());
                event.event_type = event_type.trim().to_string();
                break;
            }
        }

        if let None = found_event {
            return Err((404, String::from("Error: event not found")));
        }

        Ok(())
    }

    pub fn update_description(
        all_events: &mut Vec<Event>,
        id: &String,
        description: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_event: Option<Event> = None;

        if !String::from(description)
            .chars()
            .all(|c| c != ';' && c != '\n')
        {
            return Err((
                400,
                String::from("Error: description contains an invalid character"),
            ));
        }

        if String::from(description.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: description does not contain enough characters"),
            ));
        } else if String::from(description.trim()).len() > 1000 {
            return Err((
                400,
                String::from("Error: description contains too many characters"),
            ));
        }

        for event in all_events.iter_mut() {
            if event.id == *id {
                found_event = Some(event.clone());
                event.description = description.trim().to_string();
                break;
            }
        }

        if let None = found_event {
            return Err((404, String::from("Error: event not found")));
        }

        Ok(())
    }

    pub fn update_timestamp(
        all_events: &mut Vec<Event>,
        id: &String,
    ) -> Result<(), (usize, String)> {
        let mut found_event: Option<Event> = None;
        let timestamp = Utc::now().to_string();

        if !timestamp.chars().all(|c| c != ';' && c != '\n') {
            return Err((
                400,
                String::from("Error: timestamp contains an invalid character"),
            ));
        }

        for event in all_events.iter_mut() {
            if event.id == *id {
                found_event = Some(event.clone());
                event.timestamp = timestamp.trim().to_string();
                break;
            }
        }

        if let None = found_event {
            return Err((404, String::from("Error: event not found")));
        }

        Ok(())
    }

    pub fn update_redirect(
        all_events: &mut Vec<Event>,
        id: &String,
        redirect: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_event: Option<Event> = None;

        if !String::from(redirect)
            .chars()
            .all(|c| c != ';' && c != '\n')
        {
            return Err((
                400,
                String::from("Error: redirect contains an invalid character"),
            ));
        }

        if String::from(redirect.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: redirect does not contain enough characters"),
            ));
        } else if String::from(redirect.trim()).len() > 200 {
            return Err((
                400,
                String::from("Error: redirect contains too many characters"),
            ));
        }

        for event in all_events.iter_mut() {
            if event.id == *id {
                found_event = Some(event.clone());
                event.redirect = redirect.trim().to_string();
                break;
            }
        }

        if let None = found_event {
            return Err((404, String::from("Error: event not found")));
        }

        Ok(())
    }

    pub fn delete(all_events: &mut Vec<Event>, id: &str) -> Result<(), (usize, String)> {
        let mut found_event: Option<Event> = None;

        for event in all_events.iter_mut() {
            if event.id == id.to_string() {
                found_event = Some(event.clone());
                break;
            }
        }

        if let None = found_event {
            return Err((404, String::from("Error: event not found")));
        }

        let updated_events: Vec<Event> = all_events
            .iter_mut()
            .filter(|event| event.id != *id)
            .map(|event| Event {
                id: event.id.clone(),
                event_type: event.event_type.clone(),
                description: event.description.clone(),
                timestamp: event.timestamp.clone(),
                redirect: event.redirect.clone(),
            })
            .collect::<Vec<Event>>();

        *all_events = updated_events;

        Ok(())
    }

    pub fn to_string(event: Event) -> String {
        format!(
            "{};{};{};{};{}",
            event.id, event.event_type, event.description, event.timestamp, event.redirect
        )
    }

    pub fn from_string(event_str: &str) -> Event {
        let current_event = event_str.split(";").collect::<Vec<&str>>();

        Event::create_no_check(
            current_event[0],
            current_event[1],
            current_event[2],
            current_event[3],
            current_event[4],
        )
    }
}

pub fn stringify_events(events: &Vec<Event>) -> String {
    let mut stringified_events = String::new();

    for event in events {
        stringified_events = format!(
            "{}{}{}",
            stringified_events,
            if stringified_events.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Event::to_string(event.clone()),
        );
    }

    stringified_events
}

pub fn unwrap_events(all_events_raw: String) -> Vec<Event> {
    let individual_events = all_events_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_events: Vec<Event> = Vec::<Event>::new();

    for event in individual_events {
        let tmp_event = Event::from_string(event);
        final_events.push(tmp_event);
    }

    final_events
}

pub fn fetch_all_events(path: String, encryption_key: &String) -> Vec<Event> {
    let all_events_raw = fetch_file(path.clone(), encryption_key);

    let individual_events = all_events_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_events: Vec<Event> = Vec::<Event>::new();

    for event in individual_events {
        let tmp_event = Event::from_string(event);
        final_events.push(tmp_event);
    }

    final_events
}

pub fn save_all_events(events: &Vec<Event>, path: String, encryption_key: &String) {
    let mut stringified_events = String::new();

    for event in events {
        stringified_events = format!(
            "{}{}{}",
            stringified_events,
            if stringified_events.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Event::to_string(event.clone()),
        );
    }

    save_file(path, stringified_events, encryption_key);
    println!("Events saved!");
}
