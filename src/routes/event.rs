use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::event::Event;
use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_fetch_all_events, auto_fetch_all_mappings, auto_fetch_all_users, auto_save_all_events,
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
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let all_events = match auto_fetch_all_events(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching events"});
        }
    };

    let amount = all_events.len();
    let processed_events = paginate(all_events, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Events successfully fetched!", "events": processed_events, "amount": amount});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EventFetchInput {
    uid: String,
    event_id: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<EventFetchInput>, token: Token) -> Value {
    let uid = &data.uid;
    let event_id = &data.event_id;

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
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let all_events = match auto_fetch_all_events(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching events"});
        }
    };

    let event = match Event::get(&all_events, event_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Event with this event_id found"})
        }
    };

    return json!({"status": 200, "message": "Event successfully fetched!", "event": event});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteEventInput {
    uid: String,
    event_id: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteEventInput>, token: Token) -> Value {
    let uid = &data.uid;
    let event_id = &data.event_id;

    match verify_jwt(uid.clone(), token.0).await {
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

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    if !Event::exist(&all_events, event_id) {
        return json!({"status": 404, "message": "Error: No Event with this event_id found"});
    }

    match Event::delete(&mut all_events, event_id) {
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
