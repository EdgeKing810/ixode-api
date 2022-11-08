use rocket::delete;
use rocket::serde::json::{json, Value};

use crate::components::event::Event;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    event::auto_fetch_all_events, event::auto_save_all_events, mapping::auto_fetch_all_mappings,
    user::auto_fetch_all_users,
};

#[delete("/delete?<uid>&<event_id>")]
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
    let mut all_events = match auto_fetch_all_events(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching events"});
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

    if !Event::exist(&all_events, &passed_event_id) {
        return json!({"status": 404, "message": "Error: No Event with this event_id found"});
    }

    match Event::delete(&mut all_events, &passed_event_id) {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    match auto_save_all_events(&mappings, &all_events) {
        Ok(_) => return json!({"status": 200, "message": "Event successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
