use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_create_event, auto_fetch_all_collections, auto_fetch_all_data, auto_fetch_all_mappings,
    auto_fetch_all_projects, auto_fetch_all_users, auto_save_all_collections, auto_save_all_data,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CollectionFetchInput {
    uid: String,
    project_id: String,
}

#[post("/fetch", format = "json", data = "<data>")]
pub async fn fetch(data: Json<CollectionFetchInput>, token: Token) -> Value {
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

    let all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
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
        return json!({"status": 403, "message": "Error: Not authorized to access Collections from this Project"});
    }

    let allowed_collections = Collection::get_all(&all_collections, project_id);

    return json!({"status": 200, "message": "Collections successfully fetched!", "collections": allowed_collections});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CollectionFetchOneInput {
    uid: String,
    project_id: String,
    collection_id: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<CollectionFetchOneInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;

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

    let all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
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
        return json!({"status": 403, "message": "Error: Not authorized to access Collections from this Project"});
    }

    let collection = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(col) => col,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    return json!({"status": 200, "message": "Collection successfully fetched!", "collection": collection});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CollectionInput {
    id: String,
    project_id: String,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateCollectionInput {
    uid: String,
    collection: CollectionInput,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn create(data: Json<CreateCollectionInput>, token: Token) -> Value {
    let uid = &data.uid;
    let collection = &data.collection;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
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

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, &collection.project_id) {
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
        return json!({"status": 403, "message": "Error: Not authorized to create Collections in this Project"});
    }

    match Collection::create(
        &mut all_collections,
        &collection.id,
        &collection.project_id,
        &collection.name,
        &collection.description,
    ) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "collection_create",
        format!(
            "A new collection named col[{}] was created under pro[{}] by usr[{}]",
            collection.id, collection.project_id, uid
        ),
        format!(
            "/project/{}/collection/{}",
            collection.project_id, collection.id
        ),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Collection successfully created!"}),
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
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateCollectionInput {
    uid: String,
    project_id: String,
    collection_id: String,
    change: UpdateType,
    data: String,
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<UpdateCollectionInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let change = &data.change;
    let data = &data.data;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
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
        return json!({"status": 403, "message": "Error: Not authorized to update Collections in this Project"});
    }

    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

    let current_col = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(col) => col,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };

    match match change.clone() {
        UpdateType::ID => Collection::update_id(&mut all_collections, collection_id, data),
        UpdateType::NAME => Collection::update_name(&mut all_collections, collection_id, data),
        UpdateType::DESCRIPTION => {
            Collection::update_description(&mut all_collections, collection_id, data)
        }
    } {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if change.clone() == &UpdateType::ID {
        Data::bulk_update_collection_id(&mut all_data, collection_id, data);

        match auto_save_all_data(&mappings, project_id, data, &all_data) {
            Ok(_) => {}
            Err(e) => {
                return json!({"status": 500, "message": e});
            }
        }

        if let Err(e) = auto_create_event(
            &mappings,
            "collection_update_id",
            format!(
                "The id of the collection col[{}] under pro[{}] was updated from <{}> to <{}> by usr[{}]",
                data, project_id, collection_id, data, uid
            ),
            format!("/project/{}/collection/{}", project_id, data),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::NAME {
        if let Err(e) = auto_create_event(
            &mappings,
            "collection_update_name",
            format!(
                "The name of the collection col[{}] under pro[{}] was updated from <{}> to <{}> by usr[{}]",
                collection_id, project_id, current_col.name, data, uid
            ),
            format!("/project/{}/collection/{}", project_id, collection_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::DESCRIPTION {
        if let Err(e) = auto_create_event(
            &mappings,
            "collection_update_description",
            format!(
                "The description of the collection col[{}] under pro[{}] was updated by usr[{}]",
                collection_id, project_id, uid
            ),
            format!("/project/{}/collection/{}", project_id, collection_id),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Collection successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteCollectionInput {
    uid: String,
    project_id: String,
    collection_id: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteCollectionInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_collections = match auto_fetch_all_collections(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching collections"});
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
        return json!({"status": 403, "message": "Error: Not authorized to delete Collections in this Project"});
    }

    let current_col = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(col) => col,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };

    match Collection::delete(&mut all_collections, collection_id) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "collection_delete",
        format!(
            "The collection <{}> under pro[{}] was deleted by usr[{}]",
            current_col.name, project_id, uid
        ),
        format!("/project/{}", project_id),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_collections(&mappings, &all_collections) {
        Ok(_) => return json!({"status": 200, "message": "Collection successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
