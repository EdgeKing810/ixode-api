use rocket::data::ToByteUnit;
use rocket::post;
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::mod_route::RouteComponent;
use crate::components::routing::submodules::sub_body_data_type::BodyDataType;

// use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt_x, Token};
use crate::utils::{auto_fetch_all_mappings, auto_fetch_all_projects, auto_fetch_all_routes};

use rocket::http::uri::Origin;
use rocket::Data;

use serde_json::Value;

use super::x_utils::complete_route::CompleteRoute;
use super::x_utils::global_block_order::GlobalBlockOrder;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalParamData {
    key: String,
    value: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalBlockOrder {
    local_index: usize,
    names: Vec<String>,
}

pub fn validate_body_data(
    id: &str,
    data: Value,
    bdtype: BodyDataType,
    required: bool,
) -> Result<Value, (usize, String)> {
    if required && Value::Null == data {
        return Err((400, format!("Error: {} should not be undefined", id)));
    }

    if bdtype == BodyDataType::STRING {
        if let Value::String(_) = data {
            return Ok(data);
        } else if let Value::Number(num) = data.clone() {
            return Ok(Value::String(num.to_string()));
        } else {
            return Err((400, format!("Error: {} should be a string", id)));
        }
    } else if bdtype == BodyDataType::INTEGER {
        if let Value::Number(num) = data.clone() {
            if num.is_i64() {
                return Ok(data);
            } else {
                return Err((400, format!("Error: {} should be an integer", id)));
            }
        } else if let Value::String(num) = data.clone() {
            if let Ok(_) = num.parse::<i64>() {
                return Ok(data);
            } else {
                return Err((400, format!("Error: {} should be an integer", id)));
            }
        } else {
            return Err((400, format!("Error: {} should be an integer", id)));
        }
    } else if bdtype == BodyDataType::FLOAT {
        if let Value::Number(_) = data {
            return Ok(data);
        } else if let Value::String(num) = data.clone() {
            if let Ok(_) = num.parse::<f64>() {
                return Ok(data);
            } else {
                return Err((400, format!("Error: {} should be a number", id)));
            }
        } else {
            return Err((400, format!("Error: {} should be a number", id)));
        }
    } else if bdtype == BodyDataType::BOOLEAN {
        if let Value::Bool(_) = data {
            return Ok(data);
        } else if let Value::String(val) = data.clone() {
            return Ok(Value::Bool(val.to_lowercase().trim() == "true"));
        } else {
            return Err((400, format!("Error: {} should be a boolean", id)));
        }
    } else if bdtype == BodyDataType::OTHER {
        if let Value::Array(_) = data {
            return Ok(data);
        } else {
            if let Value::Object(_) = data {
                return Ok(data);
            } else {
                return Err((400, format!("Error: {} has an invalid type", id)));
            }
        }
    }

    Ok(data)
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

    let mut delimiter = "&".to_string();
    if let Some(params) = current_route.clone().unwrap().params {
        delimiter = params.delimiter;
    }

    let broken_str_params = full_query.split(&delimiter).collect::<Vec<&str>>();
    let mut all_params = Vec::<LocalParamData>::new();
    for param in broken_str_params {
        let broken_param = param.split("=").collect::<Vec<&str>>();
        if broken_param.len() == 2 {
            let key = broken_param[0].to_string();
            let value = broken_param[1].to_string();
            all_params.push(LocalParamData { key, value });
        }
    }

    if let Some(params) = current_route.clone().unwrap().params {
        for pair in params.pairs {
            for current_param in all_params.clone() {
                if current_param.key == pair.id {
                    if let Err(e) = validate_body_data(
                        &pair.id.clone(),
                        Value::String(current_param.value.clone()),
                        pair.bdtype,
                        false,
                    ) {
                        return json!({
                            "status": e.0,
                            "message": e.1
                        });
                    }

                    break;
                }
            }
        }
    }

    let mut body_data = Value::Null;
    if let Ok(bd) = serde_json::from_str::<Value>(&stream) {
        body_data = bd;
    }

    if let Some(aj) = current_route.clone().unwrap().auth_jwt {
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

    if current_route.clone().unwrap().body.len() > 0 {
        for bdata in current_route.clone().unwrap().body {
            if !body_data.is_object() {
                return json!({
                    "status": 400,
                    "message": "Error: Invalid body"
                });
            }

            if let Err(e) = validate_body_data(
                &bdata.id.clone(),
                body_data[bdata.id].clone(),
                bdata.bdtype,
                true,
            ) {
                return json!({
                    "status": e.0,
                    "message": e.1
                });
            }
        }
    }

    let mut global_blocks = Vec::<GlobalBlockOrder>::new();
    GlobalBlockOrder::process_blocks(&current_route.clone().unwrap(), &mut global_blocks);

    return json!({
        "status": 1000,
        "project_id": project_id,
        "api_path": api_path,
        "route": route,
        "params": all_params,
        "body": body_data,
        "global_block_order": GlobalBlockOrder::to_string(&global_blocks)
    });
}
