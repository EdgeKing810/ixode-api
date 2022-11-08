use rocket::get;
use rocket::serde::json::{json, Value};

use crate::components::project::Project;
use crate::components::user::{Role, User};
use crate::middlewares::paginate::paginate;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::project::auto_fetch_all_projects;
use crate::utils::user::auto_fetch_all_users;

#[get("/fetch?<uid>&<limit>&<offset>")]
pub async fn main(
    token: Token,
    uid: Option<&str>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Value {
    let passed_uid = match uid {
        Some(s) => s.to_string(),
        None => return json!({"status": 400, "message": "Error: No uid provided"}),
    };

    let passed_limit = match limit {
        Some(x) => x,
        None => 0,
    };
    let passed_offset = match offset {
        Some(x) => x,
        None => 0,
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

    let mut allowed_projects = Vec::<Project>::new();
    for project in all_projects {
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

        if allowed {
            allowed_projects.push(project);
        }
    }

    let amount = allowed_projects.len();
    let processed_projects = paginate(allowed_projects, passed_limit, passed_offset);

    return json!({"status": 200, "message": "Projects successfully fetched!", "projects": processed_projects, "amount": amount});
}
