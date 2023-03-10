use rocket::data::ToByteUnit;
use rocket::post;
use rocket::serde::json::json;
use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::mod_route::RouteComponent;
use crate::components::routing::submodules::sub_body_data_type::BodyDataType;

use crate::middlewares::token::{verify_jwt_x, Token};
use crate::utils::x::complete_route::CompleteRoute;
use crate::utils::{
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects,
    route::auto_fetch_all_routes,
};

use rocket::http::uri::Origin;
use rocket::Data;

use serde_json::Value;

use crate::utils::x::definition_store::{DefinitionData, DefinitionStore};
use crate::utils::x::global_block_order::GlobalBlockOrder;
use crate::utils::x::loop_processor::LoopObject;
use crate::utils::x::signal_processor::{obtain_signal, Signal};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocalParamData {
    pub key: String,
    pub value: String,
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
    } else if bdtype == BodyDataType::ARRAY {
        if let Value::Array(_) = data {
            return Ok(data);
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

pub fn process_block(
    current_route: &RouteComponent,
    all_definitions: &mut Vec<DefinitionStore>,
    global_blocks: &Vec<GlobalBlockOrder>,
    block: &GlobalBlockOrder,
    project_id: &str,
    current_index: usize,
    actual_body: &Value,
    all_params: &Vec<LocalParamData>,
) -> Result<Signal, (usize, String)> {
    if let Err(e) = DefinitionStore::add_definition(
        current_route,
        all_definitions,
        global_blocks,
        project_id,
        &block.name,
        block.index,
        current_index,
        actual_body,
        all_params,
    ) {
        return Err(e);
    }

    match obtain_signal(
        current_route,
        all_definitions,
        global_blocks,
        &block.name,
        block.index,
        current_index,
    ) {
        Ok(signal) => match signal {
            Signal::FAIL(status, message) => {
                return Err((status, message));
            }
            Signal::BREAK => Ok(Signal::BREAK),
            Signal::CONTINUE => Ok(Signal::CONTINUE),
            Signal::NONE => Ok(Signal::NONE),
            Signal::RETURN(data) => Ok(Signal::RETURN(data)),
        },
        Err(e) => {
            return Err(e);
        }
    }
}

#[post("/<_path..>", format = "json", data = "<data>")]
pub async fn main<'r>(
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

    let mut route_index = -1;
    for (i, c_route) in all_routes.iter().enumerate() {
        if c_route.route_path == route {
            route_index = i as isize;
            break;
        }
    }

    if route_index < 0 {
        return json!({
            "status": 404,
            "message": "Error: Route not found"
        });
    }

    let current_route = all_routes[route_index as usize].clone();

    let mut delimiter = "&".to_string();
    if let Some(params) = &current_route.params {
        delimiter = params.delimiter.clone();
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

    if let Some(params) = &current_route.params {
        for pair in &params.pairs {
            for current_param in all_params.clone() {
                if current_param.key == pair.id {
                    if let Err(e) = validate_body_data(
                        &pair.id.clone(),
                        Value::String(current_param.value.clone()),
                        pair.bdtype.clone(),
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

    if let Some(aj) = &current_route.auth_jwt {
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

    if current_route.body.len() > 0 {
        for bdata in &current_route.body {
            if !body_data.is_object() {
                return json!({
                    "status": 400,
                    "message": "Error: Invalid body"
                });
            }

            if let Err(e) = validate_body_data(
                &bdata.id.clone(),
                body_data[bdata.id.clone()].clone(),
                bdata.bdtype.clone(),
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
    GlobalBlockOrder::process_blocks(&current_route, &mut global_blocks);

    let mut all_definitions = Vec::<DefinitionStore>::new();
    let mut current_index = 0;

    loop {
        if current_index >= global_blocks.len() {
            break;
        }

        let mut current_block = global_blocks[current_index].clone();

        let current_loops = match LoopObject::detect_loops(&global_blocks, &current_block) {
            Ok(l) => l,
            Err(e) => {
                return json!({
                    "status": e.0,
                    "message": e.1
                });
            }
        };

        for cur_loop in current_loops {
            if current_index < cur_loop.start_index {
                break;
            } else if current_index >= cur_loop.end_index {
                continue;
            }

            let mut completed = false;
            let mut iterations = 0;

            while !completed {
                if iterations == 0 {
                    match process_block(
                        &current_route,
                        &mut all_definitions,
                        &global_blocks,
                        &global_blocks[cur_loop.start_index].clone(),
                        &project_id,
                        cur_loop.start_index,
                        &body_data,
                        &all_params,
                    ) {
                        Ok(s) => match s {
                            Signal::RETURN(r) => {
                                return r;
                            }
                            _ => {}
                        },
                        Err(e) => {
                            return json!({
                                "status": e.0,
                                "message": e.1
                            });
                        }
                    }
                } else {
                    let current_loop_value: Option<DefinitionStore>;

                    if let Some(val) = DefinitionStore::get_raw_definition(
                        &all_definitions,
                        &cur_loop.ref_var,
                        current_block.index,
                    ) {
                        current_loop_value = Some(val);
                    } else {
                        break;
                    }

                    let updated_loop_value = match current_loop_value.unwrap().data {
                        DefinitionData::INTEGER(i) => DefinitionData::INTEGER(i + 1),
                        DefinitionData::FLOAT(f) => DefinitionData::FLOAT(f + 1.0),
                        _ => {
                            break;
                        }
                    };

                    match DefinitionStore::update_definition_value(
                        &mut all_definitions,
                        cur_loop.start_index,
                        updated_loop_value,
                    ) {
                        Ok(_) => {}
                        Err(e) => {
                            return json!({
                                "status": e.0,
                                "message": e.1
                            });
                        }
                    }
                }

                completed = match LoopObject::check_completed(
                    &global_blocks,
                    &all_definitions,
                    &current_route,
                    &cur_loop,
                ) {
                    Ok(c) => c,
                    Err(e) => {
                        return json!({
                            "status": e.0,
                            "message": e.1
                        });
                    }
                };

                if completed {
                    current_index = cur_loop.end_index;
                    break;
                } else {
                    iterations += 1;
                    if iterations > 12 {
                        panic!();
                    }
                    for n in (cur_loop.start_index + 1)..cur_loop.end_index {
                        match process_block(
                            &current_route,
                            &mut all_definitions,
                            &global_blocks,
                            &global_blocks[n].clone(),
                            &project_id,
                            n,
                            &body_data,
                            &all_params,
                        ) {
                            Ok(s) => match s {
                                Signal::CONTINUE => break,
                                Signal::BREAK => {
                                    current_index = cur_loop.end_index;
                                    completed = true;
                                    break;
                                }
                                Signal::RETURN(r) => {
                                    return r;
                                }
                                _ => {}
                            },
                            Err(e) => {
                                return json!({
                                    "status": e.0,
                                    "message": e.1
                                });
                            }
                        }
                    }
                }
            }
        }

        if current_index >= global_blocks.len() {
            break;
        }

        current_block = global_blocks[current_index].clone();

        match process_block(
            &current_route,
            &mut all_definitions,
            &global_blocks,
            &current_block,
            &project_id,
            current_index,
            &body_data,
            &all_params,
        ) {
            Ok(s) => match s {
                Signal::RETURN(r) => {
                    return r;
                }
                _ => {}
            },
            Err(e) => {
                return json!({
                    "status": e.0,
                    "message": e.1
                });
            }
        }

        current_index += 1;
    }

    return json!({
        "status": 200
    });
}
