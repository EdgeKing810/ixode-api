use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::project::{auto_fetch_all_projects, auto_save_all_projects};
use crate::utils::user::auto_fetch_all_users;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectInput {
    id: String,
    name: String,
    description: String,
    api_path: String,
    members: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateProjectInput {
    uid: String,
    project: ProjectInput,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn main(data: Json<CreateProjectInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project = &data.project;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let exists = Project::exist(&all_projects, &project.id);

    if exists {
        return json!({"status": 403, "message": "Error: A Project with this project_id already exists"});
    }

    let mut final_members = project.members.clone();
    final_members.retain(|x| x.trim().len() > 0);
    let mut present = false;
    for member in project.members.clone() {
        if member.to_lowercase() == uid.to_string() {
            present = true;
            break;
        }
    }

    if !present {
        final_members.push(uid.clone());
    }

    match Project::create(
        &mut all_projects,
        &project.id,
        &project.name,
        &project.description,
        &project.api_path,
        final_members,
    ) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "project_create",
        format!(
            "A new project named pro[{}] was created by usr[{}]",
            project.id, uid
        ),
        format!("/project/{}", project.id),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => return json!({"status": 200, "message": "Project successfully created!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
