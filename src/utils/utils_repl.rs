use crate::components::user::Role;

use super::mapping::auto_fetch_all_mappings;

pub fn process_repl_query(query: String, uid: String, role: Role) -> (usize, String, String) {
    let invalid_query_msg = String::from(
        "Invalid query. Please enter a valid query or enter 'help' to display a list of commands.",
    );

    let all_mappings = auto_fetch_all_mappings();
    let mut mapping_keys = Vec::<String>::new();
    for m in all_mappings {
        mapping_keys.push(m.id.clone());
    }

    if query.trim().is_empty() {
        return (400, String::from("Empty query. Please enter a valid query or enter 'help' to display a list of commands."), String::new());
    }

    let queries = query.split(";").collect::<Vec<&str>>();
    let mut processed_queries = Vec::<String>::new();

    for q in queries {
        let lower_query = q.to_lowercase();
        let broken_query = q.split(" ").collect::<Vec<&str>>();
        let mut current_query = Vec::<String>::new();
        for bq in broken_query {
            let current = bq.trim().to_lowercase();
            if current.len() > 0 {
                current_query.push(String::from(current));
            }
        }

        if current_query[0] == "select" {
            if !lower_query.contains("from") {
                return (400, invalid_query_msg, String::new());
            }

            let mut keys = Vec::<String>::new();
            let source = current_query[current_query.len() - 1].clone();
            for cq in current_query {
                if cq == "select" {
                    continue;
                }
                if cq == "from" {
                    break;
                }
                keys.push(cq.replace(",", ""));
            }

            if !mapping_keys.contains(&source) {
                return (400, format!("'{}' is not a valid collection name. Please enter 'LIST COLLECTIONS' to obtain a list of valid collections.", source), String::new());
            }

            return (
                200,
                format!("Selecting '{}' from '{}'...", keys.join(","), source),
                String::new(),
            );

            // TODO: handle here
        }
        // TODO: handle here
    }

    if processed_queries.len() <= 0 {
        return (400, invalid_query_msg, String::new());
    }

    (200, String::from("Successful query"), query)
}
