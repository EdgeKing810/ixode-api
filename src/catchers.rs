use std::collections::HashMap;

use rocket::{
    catch,
    serde::json::{json, Value},
    Request,
};
use rocket_dyn_templates::Template;

#[catch(401)]
pub fn unauthorized() -> Value {
    json!({
        "status": "401",
        "message": "Error: Missing JWT Bearer Header"
    })
}

#[catch(400)]
pub fn bad_request() -> Value {
    json!({
        "status": "400",
        "message": "Error: Not enough information supplied"
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut context = HashMap::new();
    context.insert("uri", req.uri());

    Template::render("error/404", context)
}

#[catch(422)]
pub fn malformed_request() -> Value {
    json!({
        "status": "422",
        "message": "Error: Wrongly formed request"
    })
}
