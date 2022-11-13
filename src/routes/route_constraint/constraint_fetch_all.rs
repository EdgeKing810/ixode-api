use rocket::get;
use rocket::serde::json::{json, Value};

use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::constraint::auto_fetch_all_constraints;
use crate::utils::mapping::auto_fetch_all_mappings;

#[get("/fetch?<uid>&<limit>&<offset>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    limit: Option<usize>,
    offset: Option<usize>,
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

    let all_constraints = match auto_fetch_all_constraints(&mappings) {
        Ok(c) => c,
        Err(_) => return json!({"status": 500, "message": "Error: Failed fetching constraints"}),
    };

    let amount = all_constraints.len();
    let processed_constraints = paginate(all_constraints, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Constraints successfully fetched!", "constraints": processed_constraints, "amount": amount});
}
