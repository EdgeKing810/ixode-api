use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::routing::mod_route::RouteComponent;
use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_create_event, auto_fetch_all_collections, auto_fetch_all_data, auto_fetch_all_mappings,
    auto_fetch_all_projects, auto_fetch_all_routes, auto_fetch_all_users,
    auto_save_all_collections, auto_save_all_data, auto_save_all_projects, auto_save_all_routes,
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
    let processed_projects = paginate(allowed_projects, passed_limit, passed_offset);

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

#[derive(Serialize, Deserialize, PartialEq)]
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

    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(c) => c,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
        }
    };

    let mut all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(r) => r,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let mut all_project_data = Vec::<Data>::new();
    for col in all_collections.iter() {
        if col.project_id == *project_id {
            let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &col.id) {
                Ok(u) => u,
                _ => {
                    return json!({"status": 500, "message": "Error: Failed fetching data"});
                }
            };
            all_project_data.append(&mut all_data);
        }
    }

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
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if change.clone() == &UpdateType::ID {
        Data::bulk_update_project_id(&mut all_project_data, project_id, data);

        for route in all_routes.clone().iter() {
            match RouteComponent::update_project_id(&mut all_routes, &route.route_id, data) {
                Err(e) => return json!({"status": e.0, "message": e.1}),
                _ => {}
            }
        }

        for col in all_collections.clone().iter() {
            if col.project_id == *project_id {
                match Collection::update_project_id(&mut all_collections, &col.id, data) {
                    Err(e) => return json!({"status": e.0, "message": e.1}),
                    _ => {}
                }

                let current_data = all_project_data
                    .iter()
                    .filter(|d| d.collection_id == col.id)
                    .cloned()
                    .collect::<Vec<Data>>();

                match auto_save_all_data(&mappings, &data, &col.id, &current_data) {
                    Ok(_) => {}
                    Err(e) => {
                        return json!({"status": 500, "message": e});
                    }
                }
            }
        }

        match auto_save_all_routes(&data, &all_routes) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }

        match auto_save_all_collections(&mappings, &all_collections) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_id",
            format!(
                "The id of the project pro[{}] was updated from <{}> to <{}> by usr[{}]",
                data, project_id, data, uid
            ),
            format!("/project/{}", data),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::NAME {
        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_name",
            format!(
                "The name of the project pro[{}] was updated from <{}> to <{}> by usr[{}]",
                project_id, project.name, data, uid
            ),
            format!("/project/{}", project_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::DESCRIPTION {
        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_description",
            format!(
                "The description of the project pro[{}] was updated by usr[{}]",
                project_id, uid
            ),
            format!("/project/{}", project_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::APIPATH {
        if let Err(e) = auto_create_event(
            &mappings,
            "project_update_api_path",
            format!(
                "The api_path of the project pro[{}] was updated from <{}> to <{}> by usr[{}]",
                project_id, project.api_path, data, uid
            ),
            format!("/project/{}", project_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
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
        Err(e) => return json!({"status": e.0, "message": e.1}),
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

    if let Err(e) = auto_create_event(
        &mappings,
        "project_delete",
        format!("The project <{}> was deleted by usr[{}]", project.name, uid),
        format!("/projects"),
    ) {
        return json!({"status": e.0, "message": e.1});
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
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "project_add_member",
        format!(
            "The user usr[{}] was added to the project pro[{}] by usr[{}]",
            target_uid, project_id, uid
        ),
        format!("/project/{}", project_id),
    ) {
        return json!({"status": e.0, "message": e.1});
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
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "project_remove_member",
        format!(
            "The user usr[{}] was removed from the project pro[{}] by usr[{}]",
            target_uid, project_id, uid
        ),
        format!("/project/{}", project_id),
    ) {
        return json!({"status": e.0, "message": e.1});
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
