use rocket::get;

#[get("/hello/world")]
pub fn main() -> &'static str {
    "Hello, world!"
}
