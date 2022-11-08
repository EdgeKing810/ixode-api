use rocket::patch;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::media::Media;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    mapping::auto_fetch_all_mappings, media::auto_fetch_all_medias, media::auto_save_all_medias,
    user::auto_fetch_all_users,
};

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

#[patch("/update", format = "json", data = "<data>")]
pub async fn main(data: Json<UpdateMediaInput>, token: Token) -> Value {
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
