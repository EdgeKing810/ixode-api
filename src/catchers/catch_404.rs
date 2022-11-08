use std::collections::HashMap;

use rocket::{catch, Request};
use rocket_dyn_templates::Template;

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    let mut context = HashMap::new();
    context.insert("uri", req.uri());

    Template::render("error/404", context)
}
