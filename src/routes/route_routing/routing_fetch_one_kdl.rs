use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::project::Project;
use crate::components::routing::mod_route::RouteComponent;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects,
    route::auto_fetch_all_routes, user::auto_fetch_all_users,
};

#[get("/fetch/one/kdl?<uid>&<project_id>&<route_id>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    project_id: Option<&str>,
    route_id: Option<&str>,
) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_project_id = match project_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No project_id provided"}),
    };

    let passed_route_id = match route_id {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No route_id provided"}),
    };

    match verify_jwt(passed_uid.clone(), token.0).await {
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

    let current_user = User::get(&users, &passed_uid).unwrap();

    let all_projects = match auto_fetch_all_projects(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching projects"});
        }
    };

    let project = match Project::get(&all_projects, &passed_project_id) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Project with this project_id found"})
        }
    };

    let members = project.members.clone();
    let mut allowed = false;

    if current_user.role != Role::ROOT {
        for member in members {
            if member.to_lowercase() == passed_uid {
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

    let all_routes = match auto_fetch_all_routes(&passed_project_id) {
        Ok(d) => d,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching routes"});
        }
    };

    let current_route = match RouteComponent::get(&all_routes, &passed_project_id, &passed_route_id)
    {
        Ok(r) => r,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Route with this route_id found"})
        }
    };

    let current_route_kdl = RouteComponent::to_string(current_route);

    return json!({"status": 200, "message": "Route successfully fetched!", "route": current_route_kdl, "route_id": &passed_route_id});
}
