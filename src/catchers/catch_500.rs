use std::collections::HashMap;

use rocket::{catch, Request};
use rocket_dyn_templates::Template;

#[catch(500)]
pub fn internal_server_error(req: &Request<'_>) -> Template {
    let mut context = HashMap::new();
    context.insert("uri", req.uri());

    Template::render("error/500", context)
}
