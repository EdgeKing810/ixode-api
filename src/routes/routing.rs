use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::project::Project;
use crate::components::routing::mod_route::RouteComponent;
use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    auto_create_event, auto_fetch_all_mappings, auto_fetch_all_projects, auto_fetch_all_routes,
    auto_fetch_all_users, auto_save_all_routes,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RouteFetchInput {
    uid: String,
    project_id: String,
}

#[post("/fetch?<limit>&<offset>", format = "json", data = "<data>")]
pub async fn fetch(
    data: Json<RouteFetchInput>,
    token: Token,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;

    let passed_limit = match limit {
        Some(x) => x,
        None => 0,
    };
    let passed_offset = match offset {
        Some(x) => x,
        None => 0,
    };

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        for member in members {
            if member.to_lowercase() == uid.to_string() {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to view Routes for this Project"});
    }

    let all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let amount = all_routes.len();
    let processed_routes = paginate(all_routes, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Routes successfully fetched!", "routes": processed_routes,"amount": amount});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RouteFetchOneInput {
    uid: String,
    project_id: String,
    route_id: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<RouteFetchOneInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let route_id = &data.route_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        for member in members {
            if member.to_lowercase() == uid.to_string() {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to view Routes for this Project"});
    }

    let all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let current_route = match RouteComponent::get(&all_routes, project_id, route_id) {
        Ok(r) => r,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Route with this route_id found"})
        }
    };

    return json!({"status": 200, "message": "Route successfully fetched!", "route": current_route, "route_id": route_id});
}

#[post("/fetch/one/kdl", format = "json", data = "<data>")]
pub async fn fetch_one_kdl(data: Json<RouteFetchOneInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let route_id = &data.route_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        for member in members {
            if member.to_lowercase() == uid.to_string() {
                allowed = true;
                break;
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to view Routes for this Project"});
    }

    let all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let current_route = match RouteComponent::get(&all_routes, project_id, route_id) {
        Ok(r) => r,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Route with this route_id found"})
        }
    };

    let current_route_kdl = RouteComponent::to_string(current_route);

    return json!({"status": 200, "message": "Route successfully fetched!", "route": current_route_kdl, "route_id": route_id});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateRouteInput {
    uid: String,
    project_id: String,
    update: bool,
    route: RouteComponent,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn create(data: Json<CreateRouteInput>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let update = &data.update;
    let tmp_route = &data.route;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(r) => r,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to create Routes for this Project"});
    }

    if RouteComponent::exists(&all_routes, &tmp_route.route_id) {
        if !*update {
            return json!({"status": 400, "message": "Error: Route with this route_id already exists"});
        }
    } else {
        if *update {
            return json!({"status": 404, "message": "Error: No Route with this route_id found"});
        }
    }

    if *update {
        if let Err(e) = RouteComponent::delete(&mut all_routes, &tmp_route.route_id) {
            return json!({"status": e.0, "message": e.1});
        }
    }

    let stringified_route = RouteComponent::to_string(tmp_route.clone());
    if let Err(e) = RouteComponent::from_string(&mut all_routes, &stringified_route) {
        return json!({"status": e.0, "message": e.1});
    }

    if let Err(e) = auto_create_event(
        &mappings,
        if *update {
            "route_update"
        } else {
            "route_create"
        },
        format!(
            "A new route with id <{}> was created under pro[{}] by usr[{}]",
            tmp_route.route_id, project_id, uid
        ),
        format!("/route/p/{}/r/{}", project_id, tmp_route.route_id),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_routes(&project_id, &all_routes) {
        Ok(_) => {
            return json!({"status": 200, "message": format!("Route successfully {}!", if *update { "updated" } else { "created" }), "route_id": tmp_route.route_id})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConvertRouteBlocks {
    uid: String,
    project_id: String,
    route: RouteComponent,
}

#[post("/convert/blocks", format = "json", data = "<data>")]
pub async fn convert_blocks(data: Json<ConvertRouteBlocks>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let tmp_route = &data.route;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_routes = Vec::<RouteComponent>::new();

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to work with Routes for this Project"});
    }

    let stringified_route = RouteComponent::to_string(tmp_route.clone());
    if let Err(e) = RouteComponent::from_string(&mut all_routes, &stringified_route) {
        return json!({"status": e.0, "message": e.1, "success": false });
    }

    return json!({"status": 200, "message": "Route blocks successfully converted to KDL!", "route": stringified_route, "success": true });
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConvertRouteKDL {
    uid: String,
    project_id: String,
    route: String,
}

#[post("/convert/kdl", format = "json", data = "<data>")]
pub async fn convert_kdl(data: Json<ConvertRouteKDL>, token: Token) -> Value {
    let uid = &data.uid;
    let project_id = &data.project_id;
    let tmp_route = &data.route;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_routes = Vec::<RouteComponent>::new();

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to work with Routes for this Project"});
    }

    let converted_route = match RouteComponent::from_string(&mut all_routes, &tmp_route) {
        Ok(r) => r,
        Err(e) => {
            return json!({"status": e.0, "message": e.1, "success": false });
        }
    };

    return json!({"status": 200, "message": "Route KDL successfully converted to blocks!", "route": converted_route, "success": true });
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteRouteInput {
    uid: String,
    route_id: String,
    project_id: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteRouteInput>, token: Token) -> Value {
    let uid = &data.uid;
    let route_id = &data.route_id;
    let project_id = &data.project_id;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut all_routes = match auto_fetch_all_routes(&project_id) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        if current_user.role == Role::ADMIN {
            for member in members {
                if member.to_lowercase() == uid.to_string() {
                    allowed = true;
                    break;
                }
            }
        }
    } else {
        allowed = true;
    }

    if !allowed {
        return json!({"status": 403, "message": "Error: Not authorized to delete Routes in this Project"});
    }

    if let Err(e) = RouteComponent::delete(&mut all_routes, &route_id) {
        return json!({"status": e.0, "message": e.1});
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "route_delete",
        format!(
            "The route with id <{}> under pro[{}] was deleted by usr[{}]",
            route_id, project_id, uid
        ),
        format!("/route/p/{}", project_id),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_routes(&project_id, &all_routes) {
        Ok(_) => return json!({"status": 200, "message": "Route successfully deleted!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
