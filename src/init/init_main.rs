use std::fs;

use crate::{
    components::{
        collection::Collection, config::Config, constraint::Constraint, event::Event, media::Media,
        project::Project, user::User,
    },
    init::{
        collection::initialize_collections, config::initialize_configs,
        constraint::initialize_constraints, encryption_key::initialize_encryption_key,
        event::initialize_events, mapping::initialize_mappings, media::initialize_medias,
        project::initialize_projects, user::initialize_users,
    },
};

pub fn initialize() {
    match dotenv::dotenv() {
        Err(_) => {
            match fs::copy(".env.template", ".env") {
                Ok(_) => dotenv::dotenv().expect("Failed to read .env file"),
                Err(_) => {
                    panic!("Failed to create .env file from template");
                }
            };
        }
        _ => {}
    }

    let all_mappings = initialize_mappings();

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    match initialize_encryption_key(&all_mappings, &pass) {
        Ok(_) => println!("Encryption Key Generated!"),
        Err(e) => println!("{}", e),
    }

    let all_users: Vec<User> = initialize_users(&all_mappings);
    let all_projects: Vec<Project> = initialize_projects(&all_mappings);
    let _all_configs: Vec<Config> = initialize_configs(&all_mappings);
    let all_collections: Vec<Collection> = initialize_collections(&all_mappings);
    let _all_medias: Vec<Media> = initialize_medias(&all_mappings);
    let _all_events: Vec<Event> = initialize_events(&all_mappings);
    let all_constraints: Vec<Constraint> = initialize_constraints(&all_mappings);

    println!(
        "{:#?}",
        User::login_username(&all_users, "EdgeKing810", "Test123*")
    );

    println!("Projects: {:#?}", all_projects);

    println!("Collections: {:#?}", all_collections);

    println!("Constraints: {:#?}", all_constraints);
}
