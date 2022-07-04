use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{auto_fetch_all_mappings, auto_fetch_all_users};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TestMongoInput {
    uid: String,
    uri: String,
    name: String,
}

#[post("/test/mongo", format = "json", data = "<data>")]
pub async fn test_mongo(data: Json<TestMongoInput>, token: Token) -> Value {
    let uid = &data.uid;
    let _uri = &data.uri;
    let _name = &data.name;

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

    return json!({"status": 200, "message": "Route not ready yet."});
}
