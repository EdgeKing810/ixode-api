use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate_projects;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_fetch_all_collections, auto_fetch_all_mappings, auto_fetch_all_projects,
    auto_fetch_all_users, auto_save_all_collections, auto_save_all_projects,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NormalInput {
    uid: String,
}

#[post("/fetch?<limit>&<offset>", format = "json", data = "<data>")]
pub async fn fetch_all(
    data: Json<NormalInput>,
    token: Token,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Value {
    let uid = &data.uid;

    let passed_limit = match limit {
        Some(x) => x,
        None => 0,
    };
    let passed_offset = match offset {
        Some(x) => x,
        None => 0,
    };

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let mut allowed_projects = Vec::<Project>::new();
    for project in all_projects {
        let members = project.members.clone();
        let mut allowed = false;

        if current_user.role != Role::ROOT {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        } else {
            allowed = true;
        }

        if allowed {
            allowed_projects.push(project);
        }
    }

    let amount = allowed_projects.len();
    let processed_projects = paginate_projects(allowed_projects, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Projects successfully fetched!", "projects": processed_projects, "amount": amount});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ProjectFetchInput {
    uid: String,
    project_id: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<ProjectFetchInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        for member in members {
            if member.to_lowercase() == uid.to_string() {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to access this Project"});
    }

    return json!({"status": 200, "message": "Project successfully fetched!", "project": project});
}

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
pub async fn create(data: Json<CreateProjectInput>, token: Token) -> Value {
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
        Err(e) => return json!({"status": 500, "message": e}),
        _ => {}
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => return json!({"status": 200, "message": "Project successfully created!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub enum UpdateType {
    ID,
    NAME,
    DESCRIPTION,
    APIPATH,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateProjectInput {
    uid: String,
    project_id: String,
    change: UpdateType,
    data: String,
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<UpdateProjectInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let change = &data.change;
    let data = &data.data;

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

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to modify this Project"});
    }

    match match change.clone() {
        UpdateType::ID => Project::update_id(&mut all_projects, project_id, data),
        UpdateType::NAME => Project::update_name(&mut all_projects, project_id, data),
        UpdateType::DESCRIPTION => Project::update_description(&mut all_projects, project_id, data),
        UpdateType::APIPATH => Project::update_api_path(&mut all_projects, project_id, data),
    } {
        Err(e) => return json!({"status": 500, "message": e}),
        _ => {}
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => return json!({"status": 200, "message": "Project successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteProjectInput {
    uid: String,
    project_id: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteProjectInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;

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

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to delete this Project"});
    }

    match Project::delete(&mut all_projects, project_id) {
        Err(e) => return json!({"status": 500, "message": e}),
        _ => {}
    }

    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
        }
    };

    Collection::delete_by_project(&mut all_collections, project_id);

    match auto_save_all_collections(&mappings, &all_collections) {
        Err(e) => return json!({"status": 500, "message": e}),
        _ => {}
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => return json!({"status": 200, "message": "Project successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MemberProjectInput {
    uid: String,
    project_id: String,
    target_uid: String,
}

#[post("/member/add", format = "json", data = "<data>")]
pub async fn add_member(data: Json<MemberProjectInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let target_uid = &data.target_uid;

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

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to add users to this Project"});
    }

    match Project::add_member(&mut all_projects, project_id, target_uid) {
        Err(e) => return json!({"status": 500, "message": e}),
        _ => {}
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => return json!({"status": 200, "message": "User successfully added to Project!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[post("/member/remove", format = "json", data = "<data>")]
pub async fn remove_member(data: Json<MemberProjectInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let target_uid = &data.target_uid;

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

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to remove users to this Project"});
    }

    match Project::remove_member(&mut all_projects, project_id, target_uid) {
        Err(e) => return json!({"status": 500, "message": e}),
        _ => {}
    }

    match auto_save_all_projects(&mappings, &all_projects) {
        Ok(_) => {
            return json!({"status": 200, "message": "User successfully removed from Project!"})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
