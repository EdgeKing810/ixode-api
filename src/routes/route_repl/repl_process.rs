use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::user::User;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::repl::process_repl_query;
use crate::utils::user::auto_fetch_all_users;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReplInput {
    pub uid: String,
    pub query: String,
}

#[post("/", format = "json", data = "<data>")]
pub async fn main(data: Json<ReplInput>, token: Token) -> Value {
    let uid = &data.uid;
    let query = &data.query;

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

    let response = process_repl_query(query.clone(), uid.clone(), current_user.role.clone());

    json!({"status": response.0, "message": response.1, "data": response.2})
}
