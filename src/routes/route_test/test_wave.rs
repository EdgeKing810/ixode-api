use std::collections::HashMap;

use rocket::get;
use rocket_dyn_templates::Template;

#[get("/wave/<name>/<age>")]
pub fn main(name: &str, age: u8) -> Template {
    let mut context = HashMap::new();
    context.insert("title", "Wave 👋".to_string());
    context.insert("name", name.to_string());
    context.insert("age", age.to_string());

    Template::render("wave", context)
}
