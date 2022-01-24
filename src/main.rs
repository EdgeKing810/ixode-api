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

use std::collections::HashMap;

use init::initialize;

use rocket::{catchers, get, launch, routes};
use rocket_dyn_templates::Template;
use utils::{auto_fetch_all_mappings, get_config_value};

#[launch]
fn rocket() -> _ {
    initialize();

    rocket::build()
        .mount("/", routes![welcome])
        .mount(
            fpath("/tmp"),
            routes![routes::test::world, routes::test::wave],
        )
        .mount(
            fpath("/user"),
            routes![
                routes::user::login,
                routes::user::verify,
                routes::user::update
            ],
        )
        .register(
            "/",
            catchers![
                custom_catchers::bad_request,
                custom_catchers::unauthorized,
                custom_catchers::not_found,
                custom_catchers::malformed_request,
                custom_catchers::internal_server_error
            ],
        )
        .attach(Template::fairing())
}

#[get("/")]
fn welcome() -> Template {
    let mappings = auto_fetch_all_mappings();

    let project_name = get_config_value(&mappings, "PROJECT_NAME", "Kinesis API");
    let api_url = get_config_value(&mappings, "API_URL", "https://api.kinesis.games");
    let logo_url = get_config_value(
        &mappings,
        "LOGO_URL",
        "https://api.konnect.dev/api/v2/public/logo.png",
    );
    let documentation_url = get_config_value(&mappings, "DOCS_URL", "https://docs.kinesis.api");

    let mut context = HashMap::new();
    context.insert("project_name", project_name);
    context.insert("api_url", api_url);
    context.insert("logo_url", logo_url);
    context.insert("documentation_url", documentation_url);

    Template::render("welcome", context)
}

fn get_api_pre_path() -> String {
    let mappings = auto_fetch_all_mappings();
    get_config_value(&mappings, "API_PRE", "")
}

fn fpath(base: &str) -> String {
    format!("{}/{}", get_api_pre_path(), base)
}
