use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::media::Media;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    mapping::auto_fetch_all_mappings, media::auto_fetch_all_medias, media::auto_save_all_medias,
};

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
pub async fn main(data: Json<CreateMediaInput>, token: Token) -> Value {
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
