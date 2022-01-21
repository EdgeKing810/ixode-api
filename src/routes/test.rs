use std::collections::HashMap;

use rocket::get;
use rocket_dyn_templates::Template;

#[get("/hey/<name>")]
pub fn hello(name: &str) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Hello");
    context.insert("name", name);

    Template::render("index", context)
}

#[get("/<name>/<age>")]
pub fn wave(name: &str, age: u8) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Wave 👋".to_string());
    context.insert("name", name.to_string());
    context.insert("age", age.to_string());

    Template::render("wave", context)
}

#[get("/world")]
pub fn world() -> &'static str {
    "Hello, world!"
}
