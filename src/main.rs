#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;
extern crate argon2;
extern crate dotenv;
extern crate redis;
extern crate rocket;
extern crate rocket_multipart_form_data;

use init::main::initialize;
use rocket::serde::json::{json, Value};

#[path = "middlewares/middlewares.rs"]
mod middlewares;

#[path = "components/components.rs"]
mod components;

#[path = "routes/routes.rs"]
mod routes;

#[path = "tests/tests.rs"]
mod tests;

#[path = "utils/utils.rs"]
mod utils;

#[path = "init/init.rs"]
mod init;

#[path = "catchers/catchers.rs"]
mod catchers;

use components::constraint::stringify_constraints_debug;
use init::constraint::initialize_constraints;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use std::collections::HashMap;
use utils::{config::get_config_value, mapping::auto_fetch_all_mappings, redis::init_redis};

use rocket::{
    catchers,
    fs::{relative, FileServer},
    get,
    http::Method,
    launch, routes,
};
use rocket_dyn_templates::Template;
use std::fs;

#[launch]
fn rocket() -> _ {
    println!("Welcome to Kinesis API! 🏭\n");

    match dotenv::dotenv() {
        Err(_) => {
            match fs::copy(".env.template", ".env") {
                Ok(_) => dotenv::dotenv().expect("Failed to read .env file"),
                Err(_) => {
                    panic!("Failed to create .env file from template");
                }
            };
        }
        _ => {}
    }

    println!("{}\n", init_redis());

    let constraints = initialize_constraints(&auto_fetch_all_mappings());
    println!("\n{}", stringify_constraints_debug(&constraints));

    // let allowed_origins = AllowedOrigins::some_exact(&["https://www.acme.com"]);
    let allowed_origins = AllowedOrigins::all();

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Patch,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
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
        .mount("/init", routes![call_initialize])
        .mount("/public", FileServer::from(relative!("public")))
        .mount("/", rocket_cors::catch_all_options_routes())
        .mount(
            fpath("/tmp"),
            routes![routes::test::hello_world::main, routes::test::wave::main],
        )
        .mount(
            fpath("/user"),
            routes![
                routes::user::fetch_all::main,
                routes::user::fetch_one::main,
                routes::user::login::main,
                routes::user::login_jwt::main,
                routes::user::register::main,
                routes::user::verify::main,
                routes::user::update::main,
                routes::user::update_role::main,
                routes::user::delete::main,
            ],
        )
        .mount(
            fpath("/config"),
            routes![
                routes::config::fetch_all::main,
                routes::config::fetch_one::main,
                routes::config::add::main,
                routes::config::update::main,
                routes::config::delete::main,
            ],
        )
        .mount(
            fpath("/project"),
            routes![
                routes::project::fetch_all::main,
                routes::project::fetch_one::main,
                routes::project::create::main,
                routes::project::update::main,
                routes::project::delete::main,
                routes::project::add_member::main,
                routes::project::remove_member::main,
            ],
        )
        .mount(
            fpath("/collection"),
            routes![
                routes::collection::fetch_all::main,
                routes::collection::fetch_one::main,
                routes::collection::create::main,
                routes::collection::update::main,
                routes::collection::delete::main,
            ],
        )
        .mount(
            fpath("/structure"),
            routes![
                routes::structure::add::main,
                routes::structure::update::main,
                routes::structure::delete::main,
            ],
        )
        .mount(
            fpath("/custom_structure"),
            routes![
                routes::custom_structure::add::main,
                routes::custom_structure::update::main,
                routes::custom_structure::delete::main,
            ],
        )
        .mount(fpath("/upload"), routes![routes::upload::upload::main])
        .mount(
            fpath("/media"),
            routes![
                routes::media::fetch_all::main,
                routes::media::fetch_one::main,
                routes::media::create::main,
                routes::media::update::main,
                routes::media::delete::main,
            ],
        )
        .mount(
            fpath("/data"),
            routes![
                routes::data::fetch_all::main,
                routes::data::fetch_one::main,
                routes::data::create::main,
                routes::data::update::main,
                routes::data::delete::main,
                routes::data::publish::main,
            ],
        )
        .mount(
            fpath("/event"),
            routes![
                routes::event::fetch_all::main,
                routes::event::fetch_one::main,
                routes::event::delete::main,
            ],
        )
        .mount(
            fpath("/routing"),
            routes![
                routes::routing::fetch_all::main,
                routes::routing::fetch_one::main,
                routes::routing::fetch_one_kdl::main,
                routes::routing::create::main,
                routes::routing::convert_blocks::main,
                routes::routing::convert_kdl::main,
                routes::routing::delete::main,
            ],
        )
        .mount(
            fpath("/constraint"),
            routes![
                routes::constraint::fetch_all::main,
                routes::constraint::fetch_one::main,
                routes::constraint::update::main,
            ],
        )
        .mount(fpath("/x"), routes![routes::x::x::main])
        .mount(
            fpath("/misc"),
            routes![
                routes::misc::test_mongo::main,
                routes::misc::test_smtp::main
            ],
        )
        .register(
            "/",
            catchers![
                catchers::catch_400::bad_request,
                catchers::catch_401::unauthorized,
                catchers::catch_404::not_found,
                catchers::catch_422::malformed_request,
                catchers::catch_500::internal_server_error
            ],
        )
        .attach(Template::fairing())
        .attach(cors.clone())
        .manage(init_redis())
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

#[get("/code?<code>")]
fn call_initialize(code: Option<&str>) -> Value {
    let passed_code = String::from(match code {
        Some(x) => x,
        None => "",
    });

    let mappings = auto_fetch_all_mappings();
    let init_code = get_config_value(&mappings, "INIT_CODE", "code");

    if init_code == passed_code {
        initialize();
        return json!({"status": 200, "message": "Initialize function running..."});
    } else {
        return json!({"status": 401, "message": "Incorrect Init Code."});
    }
}

fn get_api_pre_path() -> String {
    let mappings = auto_fetch_all_mappings();
    get_config_value(&mappings, "API_PRE", "")
}

fn fpath(base: &str) -> String {
    format!("{}/{}", get_api_pre_path(), base)
}
