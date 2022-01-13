use rocket::get;

#[get("/<name>/<age>")]
pub fn wave(name: &str, age: u8) -> String {
    format!("👋 Hello, {} year old named {}!", age, name)
}

#[get("/world")]
pub fn world() -> &'static str {
    "Hello, world!"
}
