use rocket::serde::{Deserialize, Serialize};

use crate::{
    components::{
        constraint_property::ConstraintProperty,
        io::{fetch_file, save_file},
    },
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

use super::{
    blocks::{create_block::CreateBlock, fetch_block::FetchBlock, update_block::UpdateBlock},
    core::{core_auth_jwt::AuthJWT, core_body_data::BodyData, core_param_data::ParamData},
    mod_route_flow::RouteFlow,
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteComponent {
    pub route_id: String,
    pub route_path: String,
    pub project_id: String,
    pub auth_jwt: Option<AuthJWT>,
    pub body: Vec<BodyData>,
    pub params: Option<ParamData>,
    pub flow: RouteFlow,
}

impl RouteComponent {
    pub fn create(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &str,
        route_path: &str,
        project_id: &str,
        flow: RouteFlow,
    ) -> Result<(), (usize, String)> {
        let tmp_route_id = String::from("test;");
        let mut new_route_id = String::from(route_id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_route = RouteComponent {
            route_id: tmp_route_id.to_string(),
            route_path: "".to_string(),
            project_id: "".to_string(),
            auth_jwt: None,
            body: vec![],
            params: None,
            flow: flow,
        };
        all_routes.push(new_route);

        let route_id_update = Self::update_route_id(all_routes, &tmp_route_id, route_id);
        if let Err(e) = route_id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_route_id = tmp_route_id.clone();
        }

        if !has_error {
            let route_path_update = Self::update_route_path(all_routes, &new_route_id, route_path);
            if let Err(e) = route_path_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let project_id_update = Self::update_project_id(all_routes, &new_route_id, project_id);
            if let Err(e) = project_id_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_route = Self::delete(all_routes, &new_route_id);
            if let Err(e) = delete_route {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exists(all_routes: &Vec<RouteComponent>, route_id: &str) -> bool {
        let mut found = false;
        for route in all_routes.iter() {
            if route.route_id == route_id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(
        all_routes: &Vec<RouteComponent>,
        project_id: &str,
        route_id: &str,
    ) -> Result<RouteComponent, (usize, String)> {
        for route in all_routes.iter() {
            if route.route_id.to_lowercase() == route_id.to_lowercase()
                && route.project_id.to_lowercase() == project_id.to_lowercase()
            {
                return Ok(route.clone());
            }
        }

        Err((404, String::from("Error: Route not found")))
    }

    pub fn update_route_id(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        new_route_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter() {
            if route.route_id == new_route_id {
                return Err((403, String::from("Error: id is already in use")));
            }
        }

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "route_component",
            "route_id",
            new_route_id,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.route_id = final_value;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn update_route_path(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        route_path: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter() {
            if route.route_path == route_path {
                return Err((403, String::from("Error: route_path is already in use")));
            }
        }

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "route_component",
            "route_path",
            route_path,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.route_path = final_value;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn update_project_id(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        project_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "route_component",
            "project_id",
            project_id,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.project_id = final_value;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn update_auth_jwt(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        auth_jwt: Option<AuthJWT>,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.auth_jwt = auth_jwt;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn add_body_data(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        new_body_data: BodyData,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.body.push(new_body_data);
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn remove_body_data(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        body_data_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());

                let mut updated_body = Vec::<BodyData>::new();
                if body_data_index >= route.body.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of body datas present"),
                    ));
                }

                for n in 0..route.body.len() {
                    if n as u32 != body_data_index {
                        updated_body.push(route.body[n].clone());
                    }
                }

                route.body = updated_body;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn set_body(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        body: Vec<BodyData>,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.body = body;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn update_params(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        param: Option<ParamData>,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.params = param;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn update_flow(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
        flow: RouteFlow,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                route.flow = flow;
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_routes: &mut Vec<RouteComponent>,
        route_id: &String,
    ) -> Result<(), (usize, String)> {
        let mut found_route: Option<RouteComponent> = None;

        for route in all_routes.iter_mut() {
            if route.route_id == *route_id {
                found_route = Some(route.clone());
                break;
            }
        }

        if let None = found_route {
            return Err((404, String::from("Error: Route not found")));
        }

        let updated_routes: Vec<RouteComponent> = all_routes
            .iter_mut()
            .filter(|route| route.route_id != *route_id)
            .map(|route| RouteComponent {
                route_id: route.route_id.clone(),
                route_path: route.route_path.clone(),
                project_id: route.project_id.clone(),
                auth_jwt: route.auth_jwt.clone(),
                body: route.body.clone(),
                params: route.params.clone(),
                flow: route.flow.clone(),
            })
            .collect::<Vec<RouteComponent>>();

        *all_routes = updated_routes;

        Ok(())
    }

    pub fn bulk_update_collection_id(
        all_routes: &mut Vec<RouteComponent>,
        collection_id: &str,
        new_collection_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut updated_routes = all_routes.clone();
        for route in all_routes.iter_mut() {
            if let Some(auth_jwt) = route.auth_jwt.clone() {
                if auth_jwt.ref_col == collection_id {
                    let new_auth_jwt = match AuthJWT::create(
                        auth_jwt.active,
                        &auth_jwt.field,
                        new_collection_id,
                    ) {
                        Ok(auth_jwt) => auth_jwt,
                        Err(err) => return Err(err),
                    };

                    if let Err(e) = RouteComponent::update_auth_jwt(
                        &mut updated_routes,
                        &route.route_id,
                        Some(new_auth_jwt),
                    ) {
                        return Err(e);
                    }
                }
            }

            let mut updated_flow = route.flow.clone();

            let mut updated_fetch_blocks = route.flow.fetchers.clone();
            for fetch_block in route.flow.fetchers.iter() {
                if fetch_block.ref_col == collection_id {
                    if let Err(e) = FetchBlock::update_ref_col(
                        &mut updated_fetch_blocks,
                        fetch_block.global_index,
                        new_collection_id,
                    ) {
                        return Err(e);
                    }
                }
            }
            RouteFlow::set_fetch_blocks(&mut updated_flow, updated_fetch_blocks);

            let mut updated_update_blocks = route.flow.updates.clone();
            for update_block in route.flow.updates.iter() {
                if update_block.ref_col == collection_id {
                    if let Err(e) = UpdateBlock::update_ref_col(
                        &mut updated_update_blocks,
                        update_block.global_index,
                        new_collection_id,
                    ) {
                        return Err(e);
                    }
                }
            }
            RouteFlow::set_update_blocks(&mut updated_flow, updated_update_blocks);

            let mut updated_create_blocks = route.flow.creates.clone();
            for create_block in route.flow.creates.iter() {
                if create_block.ref_col == collection_id {
                    if let Err(e) = CreateBlock::update_ref_col(
                        &mut updated_create_blocks,
                        create_block.global_index,
                        new_collection_id,
                    ) {
                        return Err(e);
                    }
                }
            }
            RouteFlow::set_create_blocks(&mut updated_flow, updated_create_blocks);

            if let Err(e) =
                RouteComponent::update_flow(&mut updated_routes, &route.route_id, updated_flow)
            {
                return Err(e);
            }
        }

        *all_routes = updated_routes;

        Ok(())
    }

    pub fn stringify(all_routes: &Vec<RouteComponent>) -> String {
        let mut stringified_routes = String::new();

        for route in all_routes {
            stringified_routes = format!(
                "{}{}=============== DEFINE ROUTE ===============\n{}",
                stringified_routes,
                if stringified_routes.chars().count() > 1 {
                    "\n"
                } else {
                    ""
                },
                RouteComponent::to_string(route.clone()),
            );
        }

        stringified_routes
    }

    pub fn from_string(
        all_routes: &mut Vec<RouteComponent>,
        route_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_route = route_str.split("INIT ROUTE [").collect::<Vec<&str>>();
        if current_route.len() <= 1 {
            return Err((
                500,
                String::from("Error: Invalid route format (at the beginning of INIT ROUTE)"),
            ));
        }

        current_route = current_route[1].split("]").collect::<Vec<&str>>();
        if current_route.len() <= 1 {
            return Err((
                500,
                String::from("Error: Invalid route format (at INIT ROUTE)"),
            ));
        }

        current_route = current_route[0].split(",").collect::<Vec<&str>>();
        if current_route.len() < 3 {
            return Err((
                500,
                String::from("Error: Invalid route format (in INIT ROUTE)"),
            ));
        }

        let project_id = current_route[0];
        let route_id = current_route[1].to_string();
        let route_path = current_route[2];

        current_route = route_str.split("START FLOW").collect::<Vec<&str>>();
        if current_route.len() <= 1 {
            return Err((
                500,
                String::from("Error: Invalid route format (at the beginning of START FLOW)"),
            ));
        }

        let flow_str = current_route[1];

        current_route = current_route[0].split("\n").collect::<Vec<&str>>();

        let mut auth_jwt: Option<AuthJWT> = None;
        let mut body_data = Vec::<BodyData>::new();
        let params: Option<ParamData>;

        let mut param_arr = Vec::<String>::new();

        for line in current_route {
            if line.trim().len() <= 0 {
                continue;
            }

            if line.starts_with("DEFINE auth_jwt") {
                match AuthJWT::from_string(line) {
                    Ok(aj) => {
                        auth_jwt = Some(aj);
                    }
                    Err(e) => {
                        return Err((500, format!("Error: Invalid route format -> {}", e.1)));
                    }
                }
            } else if line.starts_with("ADD BODY pair") {
                if let Err(e) = BodyData::from_string(&mut body_data, line, false) {
                    return Err((500, format!("Error: Invalid route format -> {}", e.1)));
                }
            } else if line.starts_with("DEFINE PARAMS") || line.starts_with("ADD PARAMS") {
                param_arr.push(line.to_string());
            }
        }

        let param_arr_str = param_arr.join("\n");

        params = match ParamData::from_string(&param_arr_str) {
            Ok(p) => Some(p),
            Err(e) => {
                return Err((500, format!("Error: Invalid route format -> {}", e.1)));
            }
        };

        let flow = match RouteFlow::from_string(flow_str) {
            Ok(f) => f,
            Err(e) => {
                return Err((500, format!("Error: Invalid route format -> {}", e.1)));
            }
        };

        if let Err(e) =
            RouteComponent::create(all_routes, &route_id, route_path, project_id, flow.clone())
        {
            return Err((500, format!("Error: Invalid route format -> {}", e.1)));
        }

        if let Err(e) = RouteComponent::update_auth_jwt(all_routes, &route_id, auth_jwt) {
            return Err((500, format!("Error: Invalid route format -> {}", e.1)));
        }

        if let Err(e) = RouteComponent::set_body(all_routes, &route_id, body_data) {
            return Err((500, format!("Error: Invalid route format -> {}", e.1)));
        }

        if let Err(e) = RouteComponent::update_params(all_routes, &route_id, params) {
            return Err((500, format!("Error: Invalid route format -> {}", e.1)));
        }

        match RouteComponent::update_flow(all_routes, &route_id, flow) {
            Ok(_) => Ok(()),
            Err(e) => Err((500, format!("Error: Invalid route format -> {}", e.1))),
        }
    }

    pub fn to_string(route: RouteComponent) -> String {
        let mut route_str = format!(
            "INIT ROUTE [{},{},{}]",
            route.project_id, route.route_id, route.route_path
        );

        if let Some(auth_jwt) = route.auth_jwt {
            route_str = format!("{}\n\n{}", route_str, AuthJWT::to_string(auth_jwt));
        }

        route_str = format!(
            "{}\n\n{}",
            route_str,
            BodyData::stringify(&route.body, false)
        );

        if let Some(param) = route.params {
            route_str = format!("{}\n\n{}", route_str, ParamData::to_string(param));
        }

        route_str = format!(
            "{}\nSTART FLOW{}",
            route_str,
            RouteFlow::to_string(route.flow)
        );

        route_str
    }
}

pub fn stringify_routes(all_routes: &Vec<RouteComponent>) -> String {
    RouteComponent::stringify(all_routes)
}

pub fn unwrap_routes(all_routes_raw: String) -> Vec<RouteComponent> {
    let individual_routes = all_routes_raw
        .split("=============== DEFINE ROUTE ===============")
        .filter(|line| line.chars().count() >= 3);

    let mut final_routes: Vec<RouteComponent> = Vec::<RouteComponent>::new();

    for route in individual_routes {
        if let Err(e) = RouteComponent::from_string(&mut final_routes, route) {
            println!("Error while unwrapping route: {}", e.1);
        }
    }

    final_routes
}

pub fn fetch_all_routes(path: String, encryption_key: &String) -> Vec<RouteComponent> {
    let all_routes_raw = fetch_file(path.clone(), encryption_key);
    let final_routes = unwrap_routes(all_routes_raw);
    final_routes
}

pub fn save_all_routes(all_routes: &Vec<RouteComponent>, path: String, encryption_key: &String) {
    let stringified_routes = stringify_routes(all_routes);
    save_file(path, stringified_routes, encryption_key);
    println!("Routes saved!");
}
