#![allow(dead_code)]

use io::remove_file;
use mappings::{fetch_all_mappings, get_file_name, save_all_mappings, Mapping};
use project::{fetch_all_projects, save_all_projects, Project};
use user::{fetch_all_users, User};

mod io;
mod mappings;
mod project;
mod tests;
mod user;

const MAPPINGS_PATH: &str = "data/mappings.txt";

fn main() {
    initialize();
}

fn initialize() {
    let all_mappings = initialize_mappings();
    let mut all_users: Vec<User> = initialize_users(&all_mappings);
    let mut all_projects: Vec<Project> = initialize_projects(&all_mappings);

    println!(
        "{:#?}",
        User::login(&mut all_users, "EdgeKing810", "Test123*")
    );

    let create_project = Project::create(
        &mut all_projects,
        "konnect",
        "Konnect - Social Media",
        "A next-gen social media.",
        "/api/v2/konnect",
    );
    if let Err(e) = create_project {
        println!("{}", e);
    }

    save_all_projects(&all_projects, "data/projects.txt");

    println!("Projects: {:#?}", all_projects);
}

fn initialize_mappings() -> Vec<Mapping> {
    let mut fetched_mappings = fetch_all_mappings(MAPPINGS_PATH);

    let user_mapping = Mapping::create(&mut fetched_mappings, "users", "data/users.txt");
    if let Err(e) = user_mapping {
        println!("{}", e);
    }

    let project_mapping = Mapping::create(&mut fetched_mappings, "projects", "data/projects.txt");
    if let Err(e) = project_mapping {
        println!("{}", e);
    }

    save_all_mappings(&fetched_mappings, MAPPINGS_PATH);
    fetched_mappings
}

fn initialize_users(mappings: &Vec<Mapping>) -> Vec<User> {
    let all_users_path = get_file_name("users", mappings);
    let mut all_users = Vec::<User>::new();

    if let Ok(path) = all_users_path {
        let fetched_users = fetch_all_users(path.clone());
        all_users = fetched_users;
    }

    all_users
}

fn initialize_projects(mappings: &Vec<Mapping>) -> Vec<Project> {
    let all_projects_path = get_file_name("projects", mappings);
    let mut all_projects = Vec::<Project>::new();

    if let Ok(path) = all_projects_path {
        let fetched_projects = fetch_all_projects(path.clone());
        all_projects = fetched_projects;
    }

    all_projects
}

fn reset_db(all_mappings: Vec<Mapping>) {
    remove_file(MAPPINGS_PATH.to_string());
    for mapping in all_mappings.iter() {
        remove_file(mapping.get_file_name());
    }
}
