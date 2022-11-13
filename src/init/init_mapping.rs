use crate::{
    components::mappings::{fetch_all_mappings, save_all_mappings, Mapping},
    utils::io::{auto_create_directory, get_root_data_dir},
};

pub fn initialize_mappings() -> Vec<Mapping> {
    let root_dir = get_root_data_dir();
    let mappings_path = format!("{}{}", root_dir, "/data/mappings.txt");

    let mut fetched_mappings = fetch_all_mappings(&mappings_path, &String::new());

    auto_create_directory("/");
    auto_create_directory("/data");
    auto_create_directory("/data/projects");

    if !Mapping::exist(&fetched_mappings, "users") {
        let user_mapping = Mapping::create(&mut fetched_mappings, "users", "data/users.txt");
        if let Err(e) = user_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "projects") {
        let project_mapping =
            Mapping::create(&mut fetched_mappings, "projects", "data/projects.txt");
        if let Err(e) = project_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "configs") {
        let config_mapping = Mapping::create(&mut fetched_mappings, "configs", "data/configs.txt");
        if let Err(e) = config_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "collections") {
        let collection_mapping =
            Mapping::create(&mut fetched_mappings, "collections", "data/collections.txt");
        if let Err(e) = collection_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "medias") {
        let media_mapping = Mapping::create(&mut fetched_mappings, "medias", "data/medias.txt");
        if let Err(e) = media_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "encryption_key") {
        let encryption_key_mapping = Mapping::create(
            &mut fetched_mappings,
            "encryption_key",
            "data/encryption_key.txt",
        );
        if let Err(e) = encryption_key_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "events") {
        let event_mapping = Mapping::create(&mut fetched_mappings, "events", "data/events.txt");
        if let Err(e) = event_mapping {
            println!("{}", e);
        }
    }

    if !Mapping::exist(&fetched_mappings, "constraints") {
        let constraint_mapping =
            Mapping::create(&mut fetched_mappings, "constraints", "data/constraints.txt");
        if let Err(e) = constraint_mapping {
            println!("{}", e);
        }
    }

    save_all_mappings(&fetched_mappings, &mappings_path, &String::from(""));
    fetched_mappings
}
