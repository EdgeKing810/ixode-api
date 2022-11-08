use std::collections::HashMap;

use rocket::{catch, Request};
use rocket_dyn_templates::Template;

#[catch(401)]
pub fn unauthorized(req: &Request<'_>) -> Template {
    let mut context = HashMap::new();
    context.insert("uri", req.uri());

    Template::render("error/401", context)
}
