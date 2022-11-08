use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::media::Media;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{mapping::auto_fetch_all_mappings, media::auto_fetch_all_medias};

#[get("/fetch/one?<uid>&<media_id>")]
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

    let all_medias = match auto_fetch_all_medias(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching medias"});
        }
    };

    let media = match Media::get(&all_medias, &passed_media_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Media with this media_id found"})
        }
    };

    return json!({"status": 200, "message": "Media successfully fetched!", "media": media});
}
