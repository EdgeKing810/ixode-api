use crate::{
    components::{
        event::{fetch_all_events, save_all_events, Event},
        mappings::{get_file_name, Mapping},
    },
    utils::encryption_key::get_encryption_key,
};

pub fn initialize_events(mappings: &Vec<Mapping>) -> Vec<Event> {
    let all_events_path = get_file_name("events", mappings);
    let mut all_events = Vec::<Event>::new();

    if let Err(e) = all_events_path {
        println!("{}", e);
        return all_events;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_events = fetch_all_events(
        all_events_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    save_all_events(
        &all_events,
        all_events_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_events
}
