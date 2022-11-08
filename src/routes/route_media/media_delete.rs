use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::media::Media;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    mapping::auto_fetch_all_mappings, media::auto_fetch_all_medias, media::auto_save_all_medias,
    user::auto_fetch_all_users,
};

#[delete("/delete?<uid>&<media_id>")]
pub async fn main(token: Token, uid: Option<&str>, media_id: Option<&str>) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_media_id = match media_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No media_id provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
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

    let current_user = User::get(&users, &passed_uid).unwrap();
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    if !Media::exist(&all_medias, &passed_media_id) {
        return json!({"status": 404, "message": "Error: No Media with this media_id found"});
    }

    match Media::delete(&mut all_medias, &passed_media_id) {
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
