use crate::components::collection::{fetch_all_collections, save_all_collections, Collection};
use crate::components::config::{fetch_all_configs, save_all_configs, Config};
use crate::components::custom_structures::CustomStructure;
use crate::components::encryption::{fetch_encryption_key, save_encryption_key, EncryptionKey};
use crate::components::mappings::{fetch_all_mappings, get_file_name, save_all_mappings, Mapping};
use crate::components::media::{fetch_all_medias, save_all_medias, Media};
use crate::components::project::{fetch_all_projects, save_all_projects, Project};
use crate::components::structures::Structure;
use crate::components::user::{fetch_all_users, save_all_users, User};
use crate::utils::get_encryption_key;

use core::panic;
use std::fs;

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

    println!(
        "{:#?}",
        User::login_username(&all_users, "EdgeKing810", "Test123*")
    );

    println!("Projects: {:#?}", all_projects);

    println!("Collections: {:#?}", all_collections);
}

fn initialize_mappings() -> Vec<Mapping> {
    let mappings_path = format!(
        "{}{}",
        match std::env::var("CURRENT_PATH") {
            Ok(path) => path,
            _ => "/tmp".to_string(),
        },
        "/data/mappings.txt"
    );

    let mut fetched_mappings = fetch_all_mappings(&mappings_path, &String::new());

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

    save_all_mappings(&fetched_mappings, &mappings_path, &String::from(""));
    fetched_mappings
}

fn initialize_users(mappings: &Vec<Mapping>) -> Vec<User> {
    let all_users_path = get_file_name("users", mappings);
    let mut all_users = Vec::<User>::new();

    if let Err(e) = all_users_path {
        println!("{}", e);
        return all_users;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_users = fetch_all_users(
        all_users_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    if !User::exist_username(&all_users, "EdgeKing810") {
        let create_user = User::create(
            &mut all_users,
            "Kishan",
            "Takoordyal",
            "EdgeKing810",
            "kishan@konnect.dev",
            "Test123*",
            0,
        );
        if let Err(e) = create_user {
            println!("{}", e.1);
        }
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    save_all_users(
        &all_users,
        all_users_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_users
}

fn initialize_projects(mappings: &Vec<Mapping>) -> Vec<Project> {
    let all_projects_path = get_file_name("projects", mappings);
    let mut all_projects = Vec::<Project>::new();

    if let Err(e) = all_projects_path {
        println!("{}", e);
        return all_projects;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_projects = fetch_all_projects(
        all_projects_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    if !Project::exist(&all_projects, "konnect") {
        let create_project = Project::create(
            &mut all_projects,
            "konnect",
            "Konnect - Social Media",
            "A next-gen social media.",
            "/api/v2/konnect",
            vec![],
        );
        if let Err(e) = create_project {
            println!("{}", e.1);
        }
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    save_all_projects(
        &all_projects,
        all_projects_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_projects
}

fn initialize_configs(mappings: &Vec<Mapping>) -> Vec<Config> {
    let all_configs_path = get_file_name("configs", mappings);
    let mut all_configs = Vec::<Config>::new();

    if let Err(e) = all_configs_path {
        println!("{}", e);
        return all_configs;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_configs = fetch_all_configs(
        all_configs_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    let config_keys_template: Vec<&str> = vec![
        "ENV",
        "PROJECT_NAME",
        "FRONT_URL",
        "API_URL",
        "API_PORT",
        "API_PRE",
        "WS_PORT",
        "MONGO_URI",
        "DB_NAME",
        "JWT_EXPIRE",
        "SMTP_USERNAME",
        "SMTP_PASSWORD",
        "SMTP_HOST",
        "SMTP_PORT",
        "TOKEN_KEY",
        "ENCRYPT_KEY",
        "MISC_KEY",
        "USE_REDIS",
        "REDIS_HOST",
        "REDIS_PORT",
        "UPLOAD_SIZE",
        "SHOULD_INITIALIZE",
        "CORS_WHITELIST",
    ];

    for key in config_keys_template {
        if !Config::exist(&all_configs, key) {
            let create_config = Config::create(&mut all_configs, key, "_empty");
            if let Err(e) = create_config {
                println!("{}", e.1);
            }
        }
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    save_all_configs(
        &all_configs,
        all_configs_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_configs
}

pub fn initialize_encryption_key(
    mappings: &Vec<Mapping>,
    password: &str,
) -> Result<String, String> {
    let encryption_key_path = get_file_name("encryption_key", mappings);
    let encryption_key: Result<String, String>;

    if let Err(e) = encryption_key_path {
        return Err(e);
    }

    encryption_key = fetch_encryption_key(encryption_key_path.clone().unwrap(), password);

    let mut should_continue = false;
    match encryption_key.clone() {
        Err(_) => {
            should_continue = true;
        }
        Ok(k) => {
            if k.len() < 1 {
                should_continue = true;
            }
        }
    }

    if should_continue {
        // Encryption key most likely doesn't exist yet
        let new_encryption_key = EncryptionKey::generate(32);
        let saved_encryption_key = save_encryption_key(
            new_encryption_key.0.clone(),
            password,
            &*encryption_key_path.unwrap(),
        );

        if let Err(f) = saved_encryption_key {
            return Err(String::from(f));
        }

        println!("Encryption Key Saved!");

        return Ok(new_encryption_key.0);
    }

    Ok(encryption_key.unwrap())
}

fn initialize_collections(mappings: &Vec<Mapping>) -> Vec<Collection> {
    let all_collections_path = get_file_name("collections", mappings);
    let mut all_collections = Vec::<Collection>::new();

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    if let Err(e) = all_collections_path {
        println!("{}", e);
        return all_collections;
    }

    all_collections = fetch_all_collections(
        all_collections_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    if !Collection::exist(&all_collections, "posts") {
        let create_collection = Collection::create(
            &mut all_collections,
            "posts",
            "konnect",
            "Posts",
            "To store blog posts.",
        );
        if let Err(e) = create_collection {
            println!("{}", e.1);
        }

        let mut all_structures = Vec::<Structure>::new();
        Structure::create(
            &mut all_structures,
            "title",
            "Title",
            "Title of a post",
            "text",
            "test title",
            5,
            20,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "cover_image",
            "Cover Image",
            "Cover Image picture",
            "media",
            "https://test.image.com",
            0,
            200,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "content",
            "Content",
            "Actual content of the post",
            "richtext",
            "[ Content goes here ]",
            30,
            2000,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "views",
            "Views",
            "Number of people that have viewed a post",
            "number",
            "0",
            0,
            9999,
            false,
            false,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut all_structures,
            "published",
            "Published",
            "Whether the blog post is published or not",
            "boolean",
            "false",
            0,
            5,
            false,
            false,
            "",
            true,
        )
        .unwrap();
        Collection::set_structures(&mut all_collections, &"posts".to_string(), all_structures)
            .unwrap();

        let mut all_custom_structures = Vec::<CustomStructure>::new();
        let mut tmp_structures = Vec::<Structure>::new();

        Structure::create(
            &mut tmp_structures,
            "uid",
            "UID",
            "ID of the User posting a comment",
            "uid",
            "",
            5,
            20,
            false,
            true,
            "",
            false,
        )
        .unwrap();
        Structure::create(
            &mut tmp_structures,
            "value",
            "Value",
            "Actual content of the comment",
            "text",
            "",
            1,
            100,
            false,
            false,
            "",
            false,
        )
        .unwrap();

        CustomStructure::create(
            &mut all_custom_structures,
            "comment",
            "comment",
            "To store comments",
        )
        .unwrap();
        CustomStructure::set_structures(
            &mut all_custom_structures,
            &"comment".to_string(),
            tmp_structures,
        )
        .unwrap();
        Collection::set_custom_structures(
            &mut all_collections,
            &"posts".to_string(),
            all_custom_structures,
        )
        .unwrap();

        save_all_collections(
            &all_collections,
            all_collections_path.unwrap(),
            &get_encryption_key(&mappings, &pass),
        );
    }

    all_collections
}

fn initialize_medias(mappings: &Vec<Mapping>) -> Vec<Media> {
    let all_medias_path = get_file_name("medias", mappings);
    let mut all_medias = Vec::<Media>::new();

    if let Err(e) = all_medias_path {
        println!("{}", e);
        return all_medias;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_medias = fetch_all_medias(
        all_medias_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    if !Media::exist(&all_medias, "logo") {
        let create_logo = Media::create(&mut all_medias, "logo", "logo.png");
        if let Err(e) = create_logo {
            println!("{}", e.1);
        }
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    save_all_medias(
        &all_medias,
        all_medias_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_medias
}
