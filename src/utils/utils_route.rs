use crate::components::routing::mod_route::{
    fetch_all_routes, save_all_routes, stringify_routes, unwrap_routes, RouteComponent,
};

use super::{io::get_root_data_dir, redis::get_redis_connection};

pub fn auto_fetch_all_routes(project_id: &str) -> Result<Vec<RouteComponent>, String> {
    let connection = get_redis_connection();

    if let Ok(mut con) = connection {
        let stringified_routes = match redis::pipe()
            .cmd("GET")
            .arg(format!("routes_{}", project_id))
            .query(&mut con)
        {
            Ok(d) => Some(d),
            _ => None,
        };

        if let Some(sr) = stringified_routes {
            return Ok(unwrap_routes(sr));
        }
    }

    let all_routes_path = format!(
        "{}/data/projects/{}/routes.txt",
        get_root_data_dir(),
        project_id,
    );

    let all_routes = fetch_all_routes(all_routes_path.clone(), &"".to_string());

    Ok(all_routes)
}

pub fn auto_save_all_routes(project_id: &str, routes: &Vec<RouteComponent>) -> Result<(), String> {
    let all_routes_path = format!(
        "{}/data/projects/{}/routes.txt",
        get_root_data_dir(),
        project_id
    );

    let connection = get_redis_connection();
    if let Ok(mut con) = connection {
        redis::cmd("SET")
            .arg(format!("routes_{}", project_id))
            .arg(stringify_routes(routes))
            .execute(&mut con);
    }

    save_all_routes(routes, all_routes_path, &"".to_string());

    Ok(())
}
