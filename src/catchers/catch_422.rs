use std::collections::HashMap;

use rocket::{catch, Request};
use rocket_dyn_templates::Template;

#[catch(422)]
pub fn malformed_request(req: &Request<'_>) -> Template {
    let mut context = HashMap::new();
    context.insert("uri", req.uri());

    Template::render("error/422", context)
}
