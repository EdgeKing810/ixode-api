use rocket::data::ToByteUnit;
use rocket::post;
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::mod_route::RouteComponent;
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt_x, Token};
use crate::utils::{auto_fetch_all_mappings, auto_fetch_all_projects, auto_fetch_all_routes};

use rocket::http::uri::Origin;
use rocket::http::uri::{fmt::Path, Segments};
use rocket::request::{FromParam, FromSegments};
use rocket::Data;

use serde_json::Value;

pub struct CompleteRoute {
    r: String,
}

impl<'r> FromParam<'r> for CompleteRoute {
    type Error = &'r str;

    fn from_param(r: &'r str) -> Result<Self, Self::Error> {
        if !r.chars().all(|c| {
            c.is_ascii_alphanumeric() || c == '&' || c == '!' || c == '#' || c == '-' || c == '_'
        }) {
            return Err(r);
        }

        Ok(Self { r: r.to_string() })
    }
}

impl<'r> FromSegments<'r> for CompleteRoute {
    type Error = String;

    fn from_segments(segments: Segments<'r, Path>) -> Result<Self, Self::Error> {
        let mut r = String::new();
        for segment in segments {
            r = format!("{}/{}", r, segment);
        }

        if !r.clone().chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == '&'
                || c == '!'
                || c == '#'
                || c == '-'
                || c == '_'
                || c == '?'
                || c == '/'
        }) {
            return Err(r.clone());
        }

        Ok(Self { r })
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalParamData {
    key: String,
    value: String,
}

#[post("/<_path..>", format = "json", data = "<data>")]
pub async fn handle<'r>(
    data: Data<'r>,
    _path: CompleteRoute,
    token: Token,
    uri: &Origin<'r>,
) -> rocket::serde::json::Value {
    let mut project_id = String::new();
    let mut api_path = String::new();
    let mut route = String::new();
    let stream = match data.open(10.megabytes()).into_string().await {
        Ok(s) => s.into_inner(),
        Err(_) => return json!({"status": 400, "message": "Error: Invalid body data"}),
    };

    let mut full_path = uri.clone().into_normalized().path().to_string();
    let full_query = match uri.clone().into_normalized().query() {
        Some(query) => query.to_string(),
        None => String::new(),
    };

    full_path = full_path.replace("/x", "");

    let mappings = auto_fetch_all_mappings();

    if full_path.len() <= 1 {
        return json!({
            "status": 400,
            "message": "Error: Invalid api_path & route"
        });
    }

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    for project in all_projects {
        if full_path.starts_with(&project.api_path) {
            api_path = project.api_path;
            project_id = project.id;
            route = full_path.replace(&api_path, "");
            break;
        }
    }

    if api_path.len() == 0 {
        return json!({
            "status": 400,
            "message": "Error: Invalid api_path"
        });
    }

    if route.len() == 0 {
        return json!({
            "status": 400,
            "message": "Error: Invalid route"
        });
    }

    let broken_str_params = full_query.split("&").collect::<Vec<&str>>();
    let mut all_params = Vec::<LocalParamData>::new();
    for param in broken_str_params {
        let broken_param = param.split("=").collect::<Vec<&str>>();
        if broken_param.len() == 2 {
            let key = broken_param[0].to_string();
            let value = broken_param[1].to_string();
            all_params.push(LocalParamData { key, value });
        }
    }

    let all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let mut current_route: Option<RouteComponent> = None;
    for c_route in all_routes {
        if c_route.route_path == route {
            current_route = Some(c_route);
            break;
        }
    }

    if let None = current_route {
        return json!({
            "status": 404,
            "message": "Error: Route not found"
        });
    }

    let mut body_data = Value::Null;
    if let Ok(bd) = serde_json::from_str::<Value>(&stream) {
        body_data = bd;
    }

    if let Some(aj) = current_route.unwrap().auth_jwt {
        if aj.active {
            let payload = match body_data[aj.field.clone()].as_str() {
                Some(p) => p,
                None => {
                    return json!({
                        "status": 400,
                        "message": format!("Error: Lack of a value for {}", &aj.field)
                    });
                }
            };

            if let Err(e) = verify_jwt_x(
                String::from(payload),
                token.0,
                &project_id,
                &aj.ref_col,
                &aj.field,
            )
            .await
            {
                return json!({
                    "status": e.0,
                    "message": e.1
                });
            }
        }
    }

    return json!({"status": 1000, "project_id": project_id, "api_path": api_path, "route": route, "params": all_params, "body": body_data});
}
