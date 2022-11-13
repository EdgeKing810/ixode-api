use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::constraint::Constraint;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::constraint::auto_fetch_all_constraints;
use crate::utils::mapping::auto_fetch_all_mappings;

#[get("/fetch/one?<uid>&<component_name>")]
pub async fn main(token: Token, uid: Option<&str>, component_name: Option<&str>) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_component_name = match component_name {
        Some(c) => c.to_string(),
        None => return json!({"status": 400, "message": "Error: No component_name provided"}),
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

    let constraint = match Constraint::get(&all_constraints, &passed_component_name) {
        Ok(c) => c,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Constraint with this component_name found"})
        }
    };

    return json!({"status": 200, "message": "Constraint successfully fetched!", "constraint": constraint});
}
