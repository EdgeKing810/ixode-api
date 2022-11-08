use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::project::Project;
use crate::components::routing::mod_route::RouteComponent;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ConvertRouteBlocks {
    uid: String,
    project_id: String,
    route: RouteComponent,
}

#[post("/convert/blocks", format = "json", data = "<data>")]
pub async fn main(data: Json<ConvertRouteBlocks>, token: Token) -> Value {
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
