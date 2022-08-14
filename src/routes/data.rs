use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;
use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::data_converter::{convert_from_raw, convert_to_raw, RawPair};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_fetch_all_collections, auto_fetch_all_data, auto_fetch_all_mappings,
    auto_fetch_all_projects, auto_fetch_all_users, auto_save_all_data,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DataFetchInput {
    uid: String,
    project_id: String,
    collection_id: String,
}

#[post("/fetch", format = "json", data = "<data>")]
pub async fn fetch(data: Json<DataFetchInput>, token: Token) -> Value {
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

    let collection = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
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
        return json!({"status": 403, "message": "Error: Not authorized to view Data for this Collection"});
    }

    let all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

    let current_data = Data::get_all(&all_data, project_id, collection_id);
    let mut raw_pairs = Vec::<RawPair>::new();
    let mut data_ids = Vec::<String>::new();

    // println!("data: {:#?}", current_data);

    for data in current_data {
        match convert_to_raw(&data, &collection) {
            Ok(rp) => {
                raw_pairs.push(rp);
                data_ids.push(data.id.clone());
            }
            Err(e) => {
                return json!({"status": e.0, "message": e.1});
            }
        };
    }

    return json!({"status": 200, "message": "Data successfully fetched!", "pairs": raw_pairs, "data_ids": data_ids});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DataFetchOneInput {
    uid: String,
    project_id: String,
    collection_id: String,
    data_id: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<DataFetchOneInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let data_id = &data.data_id;

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

    let collection = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
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
        return json!({"status": 403, "message": "Error: Not authorized to view Data for this Collection"});
    }

    let all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

    let current_data = match Data::get(&all_data, project_id, collection_id, data_id) {
        Ok(d) => d,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Data with this data_id found"})
        }
    };

    let raw_pair = match convert_to_raw(&current_data, &collection) {
        Ok(rp) => rp,
        Err(e) => {
            return json!({"status": e.0, "message": e.1});
        }
    };

    return json!({"status": 200, "message": "Data successfully fetched!", "pair": raw_pair, "data_id": data_id});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateDataInput {
    uid: String,
    project_id: String,
    collection_id: String,
    raw_pair: RawPair,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn create(data: Json<CreateDataInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let raw_pair = &data.raw_pair;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

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

    let collection = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::AUTHOR {
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
        return json!({"status": 403, "message": "Error: Not authorized to create Data for this Collection"});
    }

    let data_id = match convert_from_raw(&mut all_data, &collection, raw_pair, false) {
        Ok(id) => id,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };

    match auto_save_all_data(&mappings, &project_id, &collection_id, &all_data) {
        Ok(_) => {
            return json!({"status": 200, "message": "Data successfully created!", "data_id": data_id})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateDataInput {
    uid: String,
    data_id: String,
    project_id: String,
    collection_id: String,
    raw_pair: RawPair,
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<UpdateDataInput>, token: Token) -> Value {
    let uid = &data.uid;
    let data_id = &data.data_id;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let raw_pair = &data.raw_pair;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

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

    let collection = match Collection::get(&all_collections, project_id, collection_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Collection with this collection_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::AUTHOR {
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
        return json!({"status": 403, "message": "Error: Not authorized to update Data in this Collection"});
    }

    let new_data_id = match convert_from_raw(&mut all_data, &collection, raw_pair, true) {
        Ok(id) => id,
        Err(e) => return json!({"status": e.0, "message": e.1}),
    };

    match Data::delete(&mut all_data, &data_id) {
        Ok(_) => {}
        Err(e) => {
            return json!({"status": e.0, "message": e.1});
        }
    }

    match auto_save_all_data(&mappings, &project_id, &collection_id, &all_data) {
        Ok(_) => {
            return json!({"status": 200, "message": "Data successfully updated!", "data_id": new_data_id})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteDataInput {
    uid: String,
    data_id: String,
    project_id: String,
    collection_id: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteDataInput>, token: Token) -> Value {
    let uid = &data.uid;
    let data_id = &data.data_id;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

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

    if !Collection::exist(&all_collections, collection_id) {
        return json!({"status": 404, "message": "Error: No Collection with this collection_id found"});
    }

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::AUTHOR {
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
        return json!({"status": 403, "message": "Error: Not authorized to delete Data in this Collection"});
    }

    if let Err(e) = Data::delete(&mut all_data, &data_id) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_data(&mappings, &project_id, &collection_id, &all_data) {
        Ok(_) => return json!({"status": 200, "message": "Data successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PublishDataInput {
    uid: String,
    data_id: String,
    project_id: String,
    collection_id: String,
    publish: bool,
}

#[post("/publish", format = "json", data = "<data>")]
pub async fn publish(data: Json<PublishDataInput>, token: Token) -> Value {
    let uid = &data.uid;
    let data_id = &data.data_id;
    let project_id = &data.project_id;
    let collection_id = &data.collection_id;
    let publish = &data.publish;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_data = match auto_fetch_all_data(&mappings, &project_id, &collection_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching data"});
        }
    };

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

    if !Collection::exist(&all_collections, collection_id) {
        return json!({"status": 404, "message": "Error: No Collection with this collection_id found"});
    }

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::AUTHOR {
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
        return json!({"status": 403, "message": format!("Error: Not authorized to {}publish Data in this Collection", if *publish { "" } else { "un" })});
    }

    if let Err(e) = Data::update_published(&mut all_data, &data_id, *publish) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_data(&mappings, &project_id, &collection_id, &all_data) {
        Ok(_) => {
            return json!({"status": 200, "message": format!("Data successfully {}published!", if *publish { "" } else { "un" } )})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
