use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::auto_fetch_all_users;

#[get("/fetch?<uid>&<limit>&<offset>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_limit = match limit {
        Some(x) => x,
        None => 0,
    };
    let passed_offset = match offset {
        Some(x) => x,
        None => 0,
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
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let mut clean_users = users.clone();
    User::remove_passwords(&mut clean_users);
    let amount = clean_users.len();

    let processed_users = paginate(clean_users, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Users successfully fetched!", "users": processed_users, "amount": amount});
}
