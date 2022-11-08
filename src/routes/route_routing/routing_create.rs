use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::project::Project;
use crate::components::routing::mod_route::RouteComponent;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    event::auto_create_event, mapping::auto_fetch_all_mappings, project::auto_fetch_all_projects,
    route::auto_fetch_all_routes, route::auto_save_all_routes, user::auto_fetch_all_users,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateRouteInput {
    uid: String,
    project_id: String,
    update: bool,
    route: RouteComponent,
}

#[post("/create", format = "json", data = "<data>")]
pub async fn main(data: Json<CreateRouteInput>, token: Token) -> Value {
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
            "A route with id <{}> was {} under pro[{}] by usr[{}]",
            tmp_route.route_id,
            if *update { "updated" } else { "created" },
            project_id,
            uid
        ),
        format!("/routes/p/{}/r/v/{}", project_id, tmp_route.route_id),
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
