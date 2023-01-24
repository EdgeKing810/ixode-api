use crate::{
    components::io::{fetch_file, save_file},
    utils::{
        constraint::auto_fetch_all_constraints, io::auto_create_directory,
        io::auto_remove_directory, io::auto_rename_directory, mapping::auto_fetch_all_mappings,
    },
};
use rocket::serde::{Deserialize, Serialize};

use super::constraint_property::ConstraintProperty;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    description: String,
    pub api_path: String,
    pub members: Vec<String>,
}

impl Project {
    fn create_no_check(
        id: &str,
        name: &str,
        description: &str,
        api_path: &str,
        members: Vec<String>,
    ) -> Project {
        let mut final_members = members.clone();
        final_members.retain(|x| x.trim().len() > 0);

        Project {
            id: String::from(id),
            name: String::from(name),
            description: String::from(description),
            api_path: String::from(api_path),
            members: final_members,
        }
    }

    pub fn exist(all_projects: &Vec<Project>, id: &str) -> bool {
        let mut found = false;
        for project in all_projects.iter() {
            if project.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(all_projects: &Vec<Project>, project_id: &str) -> Result<Project, (usize, String)> {
        for project in all_projects.iter() {
            if project.id.to_lowercase() == project_id.to_lowercase() {
                return Ok(project.clone());
            }
        }

        Err((404, String::from("Error: Project not found")))
    }

    pub fn create(
        all_projects: &mut Vec<Project>,
        id: &str,
        name: &str,
        description: &str,
        api_path: &str,
        members: Vec<String>,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        auto_create_directory(&format!("/data/projects/{}", &tmp_id));

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_project = Project {
            id: tmp_id.clone(),
            name: "".to_string(),
            description: "".to_string(),
            api_path: "".to_string(),
            members: vec![],
        };
        all_projects.push(new_project);

        let id_update = Self::update_id(all_projects, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_id = tmp_id.clone();
        }

        if !has_error {
            let name_update = Self::update_name(all_projects, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let description_update = Self::update_description(all_projects, &new_id, description);
            if let Err(e) = description_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let api_path_update = Self::update_api_path(all_projects, &new_id, api_path);
            if let Err(e) = api_path_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let members_update = Self::update_members(all_projects, &new_id, members);
            if let Err(e) = members_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_project = Self::delete(all_projects, &new_id);
            if let Err(e) = delete_project {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn update_id(
        all_projects: &mut Vec<Project>,
        id: &String,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_project: Option<Project> = None;

        for project in all_projects.iter() {
            if project.id == new_id {
                return Err((403, String::from("Error: id is already in use")));
            }
        }

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "project", "id", new_id) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.id = final_value.clone();
                break;
            }
        }

        if let None = found_project {
            return Err((404, String::from("Error: Project not found")));
        } else {
            auto_rename_directory(
                &format!("/data/projects/{}", &id),
                &format!("/data/projects/{}", final_value),
            );
        }

        Ok(())
    }

    pub fn update_name(
        all_projects: &mut Vec<Project>,
        id: &String,
        name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_project: Option<Project> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "project", "name", name) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.name = final_value;
                break;
            }
        }

        if let None = found_project {
            return Err((404, String::from("Error: Project not found")));
        }

        Ok(())
    }

    pub fn update_description(
        all_projects: &mut Vec<Project>,
        id: &String,
        description: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_project: Option<Project> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "project",
            "description",
            description,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.description = final_value;
                break;
            }
        }

        if let None = found_project {
            return Err((404, String::from("Error: Project not found")));
        }

        Ok(())
    }

    pub fn update_api_path(
        all_projects: &mut Vec<Project>,
        id: &String,
        api_path: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_project: Option<Project> = None;

        for project in all_projects.iter() {
            if project.api_path == api_path {
                return Err((403, String::from("Error: api_path is already in use")));
            }
        }

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "project",
            "api_path",
            &api_path.to_lowercase(),
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                project.api_path = final_value;
                break;
            }
        }

        if let None = found_project {
            return Err((404, String::from("Error: Project not found")));
        }

        Ok(())
    }

    pub fn update_members(
        all_projects: &mut Vec<Project>,
        id: &String,
        members: Vec<String>,
    ) -> Result<(), (usize, String)> {
        let mut has_error = false;
        let mut last_error: (usize, String) = (500, String::new());

        for member in members.clone() {
            if member.trim().len() == 0 {
                continue;
            }

            if let Err(e) = Project::add_member(all_projects, id, &member) {
                has_error = true;
                last_error = e;
                break;
            }
        }

        return match has_error {
            false => Ok(()),
            true => Err(last_error),
        };
    }

    pub fn add_member(
        all_projects: &mut Vec<Project>,
        id: &String,
        member: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_project: Option<Project> = None;
        let mut all_members = Vec::<String>::new();

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "project",
            "members",
            &member.to_lowercase(),
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                all_members = project.members.clone();
                break;
            }
        }

        if let None = found_project {
            return Err((404, String::from("Error: Project not found")));
        }

        if let Some(pro) = found_project {
            for m in pro.members.iter() {
                if member.to_lowercase() == m.to_lowercase() {
                    return Err((
                        400,
                        String::from("Error: List of Member IDs contains duplicate(s)"),
                    ));
                }
            }
        }

        all_members.push(final_value);

        for project in all_projects.iter_mut() {
            if project.id == *id {
                project.members = all_members;
                break;
            }
        }

        Ok(())
    }

    pub fn remove_member(
        all_projects: &mut Vec<Project>,
        id: &String,
        member: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_project: Option<Project> = None;
        let mut found_member = false;
        let mut all_members = Vec::<String>::new();
        let mut filtered_members = Vec::<String>::new();

        for project in all_projects.iter_mut() {
            if project.id == *id {
                found_project = Some(project.clone());
                all_members = project.members.clone();
                break;
            }
        }

        if let None = found_project {
            return Err((404, String::from("Error: Project not found")));
        }

        if let Some(pro) = found_project {
            for m in pro.members.iter() {
                if member.to_lowercase() == m.to_lowercase() {
                    found_member = true;
                    break;
                }
            }
        }

        if !found_member {
            return Err((
                404,
                String::from("Error: No Member with this Member ID found"),
            ));
        }

        for m in all_members {
            if member.to_lowercase() != m.to_lowercase() {
                filtered_members.push(m);
                break;
            }
        }

        for project in all_projects.iter_mut() {
            if project.id == *id {
                project.members = filtered_members;
                break;
            }
        }

        Ok(())
    }

    pub fn delete(all_projects: &mut Vec<Project>, id: &String) -> Result<(), (usize, String)> {
        let mut found_project: Option<Project> = None;

        for project in all_projects.iter_mut() {
            if project.id == id.to_string() {
                found_project = Some(project.clone());
                break;
            }
        }

        if let None = found_project {
            return Err((404, String::from("Error: Project not found")));
        }

        let updated_projects: Vec<Project> = all_projects
            .iter_mut()
            .filter(|project| project.id != *id)
            .map(|project| Project {
                id: project.id.clone(),
                name: project.name.clone(),
                description: project.description.clone(),
                api_path: project.api_path.clone(),
                members: project.members.clone(),
            })
            .collect::<Vec<Project>>();

        *all_projects = updated_projects;

        auto_remove_directory(&format!("/data/projects/{}", &id));

        Ok(())
    }

    pub fn obtain_properties() -> String {
        String::from("id;name;description;api_path;members")
    }

    pub fn to_string(project: Project) -> String {
        let mut members_string = String::new();
        for member in project.members {
            members_string = format!("{}|{}", members_string, member);
        }

        format!(
            "{};{};{};{};{}",
            project.id,
            project.name,
            project
                .description
                .split("\n")
                .collect::<Vec<&str>>()
                .join("_newline_"),
            project.api_path,
            members_string
        )
    }

    pub fn from_string(project_str: &str) -> Project {
        let current_project = project_str.split(";").collect::<Vec<&str>>();
        let members = current_project[4].split("|").collect::<Vec<&str>>();
        let mut final_members = Vec::<String>::new();

        for member in members {
            final_members.push(member.to_string());
        }

        Project::create_no_check(
            current_project[0],
            current_project[1],
            &current_project[2]
                .split("_newline_")
                .collect::<Vec<&str>>()
                .join("\n"),
            current_project[3],
            final_members,
        )
    }
}

pub fn stringify_projects(projects: &Vec<Project>) -> String {
    let mut stringified_projects = String::new();

    for project in projects {
        stringified_projects = format!(
            "{}{}{}",
            stringified_projects,
            if stringified_projects.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Project::to_string(project.clone()),
        );
    }

    stringified_projects
}

pub fn unwrap_projects(all_projects_raw: String) -> Vec<Project> {
    let individual_projects = all_projects_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_projects: Vec<Project> = Vec::<Project>::new();

    for project in individual_projects {
        let tmp_project = Project::from_string(project);
        final_projects.push(tmp_project);
    }

    final_projects
}

pub fn fetch_all_projects(path: String, encryption_key: &String) -> Vec<Project> {
    let all_projects_raw = fetch_file(path.clone(), encryption_key);
    let final_projects: Vec<Project> = unwrap_projects(all_projects_raw);
    final_projects
}

pub fn save_all_projects(projects: &Vec<Project>, path: String, encryption_key: &String) {
    let stringified_projects = stringify_projects(projects);
    save_file(path, stringified_projects, encryption_key);
    println!("Projects saved!");
}
