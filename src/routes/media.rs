use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::media::Media;
use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_fetch_all_mappings, auto_fetch_all_medias, auto_fetch_all_users, auto_save_all_medias,
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

    let all_medias = match auto_fetch_all_medias(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching medias"});
        }
    };

    let amount = all_medias.len();
    let processed_medias = paginate(all_medias, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Medias successfully fetched!", "medias": processed_medias, "amount": amount});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MediaFetchInput {
    uid: String,
    media_id: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<MediaFetchInput>, token: Token) -> Value {
    let uid = &data.uid;
    let media_id = &data.media_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();

    let all_medias = match auto_fetch_all_medias(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching medias"});
        }
    };

    let media = match Media::get(&all_medias, media_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Media with this media_id found"})
        }
    };

    return json!({"status": 200, "message": "Media successfully fetched!", "media": media});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct MediaInput {
    id: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateMediaInput {
    uid: String,
    media: MediaInput,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn create(data: Json<CreateMediaInput>, token: Token) -> Value {
    let uid = &data.uid;
    let media = &data.media;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_medias = match auto_fetch_all_medias(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching medias"});
        }
    };

    let exists = Media::exist(&all_medias, &media.id);

    if exists {
        return json!({"status": 403, "message": "Error: A Media with this media_id already exists"});
    }

    match Media::create(&mut all_medias, &media.id, &media.name) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    match auto_save_all_medias(&mappings, &all_medias) {
        Ok(_) => return json!({"status": 200, "message": "Media successfully created!"}),
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
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateMediaInput {
    uid: String,
    media_id: String,
    change: UpdateType,
    data: String,
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<UpdateMediaInput>, token: Token) -> Value {
    let uid = &data.uid;
    let media_id = &data.media_id;
    let change = &data.change;
    let data = &data.data;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_medias = match auto_fetch_all_medias(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching medias"});
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

    if !Media::exist(&all_medias, media_id) {
        return json!({"status": 404, "message": "Error: No Media with this media_id found"});
    }

    match match change.clone() {
        UpdateType::ID => Media::update_id(&mut all_medias, media_id, data),
        UpdateType::NAME => Media::update_name(&mut all_medias, media_id, data),
    } {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    match auto_save_all_medias(&mappings, &all_medias) {
        Ok(_) => return json!({"status": 200, "message": "Media successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteMediaInput {
    uid: String,
    media_id: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteMediaInput>, token: Token) -> Value {
    let uid = &data.uid;
    let media_id = &data.media_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_medias = match auto_fetch_all_medias(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching medias"});
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

    if !Media::exist(&all_medias, media_id) {
        return json!({"status": 404, "message": "Error: No Media with this media_id found"});
    }

    match Media::delete(&mut all_medias, media_id) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    match auto_save_all_medias(&mappings, &all_medias) {
        Ok(_) => return json!({"status": 200, "message": "Media successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
