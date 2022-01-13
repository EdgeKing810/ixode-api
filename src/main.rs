#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;
extern crate argon2;
extern crate rocket;

mod init;
mod utils;

#[path = "components/components.rs"]
mod components;
#[path = "routes/routes.rs"]
mod routes;
#[path = "tests/tests.rs"]
mod tests;

use init::initialize;

use rocket::{launch, routes};

#[launch]
fn rocket() -> _ {
    initialize();

    rocket::build()
        .mount("/hello", routes![routes::test::world])
        .mount("/wave", routes![routes::test::wave])
}
