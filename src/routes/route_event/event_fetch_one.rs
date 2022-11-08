use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::event::Event;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    event::auto_fetch_all_events, mapping::auto_fetch_all_mappings, user::auto_fetch_all_users,
};

#[get("/fetch/one?<uid>&<event_id>")]
pub async fn main(token: Token, uid: Option<&str>, event_id: Option<&str>) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_event_id = match event_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No event_id provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
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

    let current_user = User::get(&users, &passed_uid).unwrap();
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let all_events = match auto_fetch_all_events(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching events"});
        }
    };

    let event = match Event::get(&all_events, &passed_event_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Event with this event_id found"})
        }
    };

    return json!({"status": 200, "message": "Event successfully fetched!", "event": event});
}
