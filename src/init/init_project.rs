use crate::{
    components::{
        mapping::{get_file_name, Mapping},
        project::{fetch_all_projects, save_all_projects, Project},
    },
    utils::encryption_key::get_encryption_key,
};

pub fn initialize_projects(mappings: &Vec<Mapping>) -> Vec<Project> {
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
