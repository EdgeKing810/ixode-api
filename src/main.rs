#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;
extern crate argon2;
extern crate dotenv;
extern crate rocket;

mod init;
mod utils;

#[path = "catchers.rs"]
mod custom_catchers;
#[path = "middlewares/middlewares.rs"]
mod middlewares;

#[path = "components/components.rs"]
mod components;
#[path = "routes/routes.rs"]
mod routes;
#[path = "tests/tests.rs"]
mod tests;

use init::initialize;

use rocket::{catchers, launch, routes};

#[launch]
fn rocket() -> _ {
    initialize();

    rocket::build()
        .mount("/hello", routes![routes::test::world])
        .mount("/wave", routes![routes::test::wave])
        .mount("/user", routes![routes::user::login, routes::user::verify])
        .register(
            "/",
            catchers![
                custom_catchers::bad_request,
                custom_catchers::malformed_request,
                custom_catchers::unauthorized
            ],
        )
}
