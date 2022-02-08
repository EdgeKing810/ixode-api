#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;
extern crate argon2;
extern crate dotenv;
extern crate redis;
extern crate rocket;
extern crate rocket_multipart_form_data;

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

use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::collections::HashMap;

use init::initialize;

use rocket::{
    catchers,
    fs::{relative, FileServer},
    get,
    http::Method,
    launch, routes,
};
use rocket_dyn_templates::Template;
use utils::{auto_fetch_all_mappings, get_config_value, init_redis};

#[launch]
fn rocket() -> _ {
    initialize();
    println!("{}", init_redis());

    // let allowed_origins = AllowedOrigins::some_exact(&["https://www.acme.com"]);
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        // allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::build()
        .mount("/", routes![welcome])
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount(
            fpath("/tmp"),
            routes![routes::test::world, routes::test::wave],
        )
        .mount(
            fpath("/user"),
            routes![
                routes::user::fetch_all,
                routes::user::fetch_one,
                routes::user::login,
                routes::user::verify,
                routes::user::update,
                routes::user::update_role,
                routes::user::delete,
            ],
        )
        .mount(
            fpath("/config"),
            routes![
                routes::config::fetch_all,
                routes::config::fetch_one,
                routes::config::add,
                routes::config::update,
                routes::config::delete,
            ],
        )
        .mount(
            fpath("/project"),
            routes![
                routes::project::fetch_all,
                routes::project::fetch_one,
                routes::project::create,
                routes::project::update,
                routes::project::delete,
                routes::project::add_member,
                routes::project::remove_member,
            ],
        )
        .mount(
            fpath("/collection"),
            routes![
                routes::collection::fetch,
                routes::collection::fetch_one,
                routes::collection::create,
                routes::collection::update,
                routes::collection::delete,
            ],
        )
        .mount(
            fpath("/structure"),
            routes![
                routes::structure::add,
                routes::structure::update,
                routes::structure::delete,
            ],
        )
        .mount(
            fpath("/custom_structure"),
            routes![
                routes::custom_structure::add,
                routes::custom_structure::update,
                routes::custom_structure::delete,
            ],
        )
        .mount(fpath("/upload"), routes![routes::upload::upload])
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
        .attach(cors.clone())
        .manage(utils::init_redis())
        .manage(cors)
}

#[get("/")]
fn welcome() -> Template {
    let mappings = auto_fetch_all_mappings();

    let project_name = get_config_value(&mappings, "PROJECT_NAME", "Kinesis API");
    let front_url = get_config_value(&mappings, "FRONT_URL", "https://www.kinesis.world");
    let logo_url = get_config_value(
        &mappings,
        "LOGO_URL",
        "http://127.0.0.1:8000/public/banner_purple.png",
    );
    let documentation_url = get_config_value(&mappings, "DOCS_URL", "https://docs.kinesis.world");

    let mut context = HashMap::new();
    context.insert("project_name", project_name);
    context.insert("front_url", front_url);
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
