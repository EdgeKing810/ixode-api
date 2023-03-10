use std::str::Split;

use crate::components::{
    config::{stringify_configs, Config},
    constraint::Constraint,
    constraint_property::ConstraintProperty,
    event::{stringify_events, Event},
    mapping::{stringify_mappings, Mapping},
    media::{stringify_medias, Media},
    project::{stringify_projects, Project},
    user::{stringify_users, Role, User},
};

use super::{
    config::auto_fetch_all_configs, constraint::auto_fetch_all_constraints,
    event::auto_fetch_all_events, mapping::auto_fetch_all_mappings, media::auto_fetch_all_medias,
    project::auto_fetch_all_projects, user::auto_fetch_all_users,
};

pub fn process_repl_query(
    query: String,
    _uid: String,
    role: Role,
) -> (usize, String, Vec<Vec<String>>) {
    let invalid_query_msg = String::from(
        "Invalid query. Please enter a valid query or enter 'help' to display a list of commands.",
    );

    if role != Role::ROOT {
        return (
            401,
            String::from(
                "Sorry, the REPL can only be used by users having ROOT priviledge at this moment.",
            ),
            vec![],
        );
    }

    let all_mappings = auto_fetch_all_mappings();
    let mut mapping_keys: Vec<String> = vec![String::from("mappings")];
    for m in all_mappings.clone() {
        mapping_keys.push(m.id.clone());
    }

    if query.trim().is_empty() {
        return (400, String::from("Empty query. Please enter a valid query or enter 'help' to display a list of commands."), vec![]);
    }

    let queries = query.split(";").collect::<Vec<&str>>();
    let mut processed_queries = Vec::<Vec<String>>::new();

    for q in queries {
        if q.trim().is_empty() {
            continue;
        }

        let lower_query = q.trim().to_lowercase();
        let broken_query = q.split(" ").collect::<Vec<&str>>();
        let mut current_query = Vec::<String>::new();
        for bq in broken_query.clone() {
            let current = bq.trim().to_lowercase();
            if current.len() > 0 {
                current_query.push(String::from(current));
            }
        }

        if current_query[0] == "select" {
            if !lower_query.contains("from") {
                return (400, invalid_query_msg, vec![]);
            }

            let mut keys = Vec::<String>::new();
            let mut source = String::new();
            let mut passed_from = false;

            let mut and_arguments = Vec::<(&str, &str, &str)>::new();
            let mut or_arguments = Vec::<(&str, &str, &str)>::new();
            if lower_query.contains("where") {
                let where_arguments_tmp = lower_query.split("where").collect::<Vec<&str>>()[0];
                let where_arguments = &q.trim()[where_arguments_tmp.len() + 5..];

                let and_arguments_raw = where_arguments.split("AND").collect::<Vec<&str>>();
                for aar in and_arguments_raw {
                    let or_arguments_raw = aar.split("OR").collect::<Vec<&str>>();
                    let aar_split = or_arguments_raw[0].trim().split(" ").collect::<Vec<&str>>();
                    if aar_split.len() == 3 {
                        and_arguments.push((aar_split[0], aar_split[1], aar_split[2]));
                    }

                    if or_arguments_raw.len() > 1 {
                        for (i, oar) in or_arguments_raw.iter().enumerate() {
                            if i == 0 {
                                continue;
                            }

                            let oar_split = oar.trim().split(" ").collect::<Vec<&str>>();
                            if oar_split.len() == 3 {
                                or_arguments.push((oar_split[0], oar_split[1], oar_split[2]));
                            }
                        }
                    }
                }
            }

            for cq in current_query {
                if cq == "select" {
                    continue;
                }

                if cq == "where" {
                    break;
                }

                if passed_from {
                    source = cq;
                    break;
                } else {
                    if cq == "from" {
                        passed_from = true;
                    } else {
                        for k in cq.split(",") {
                            if !k.trim().is_empty() {
                                keys.push(String::from(k));
                            }
                        }
                    }
                }
            }

            if !mapping_keys.contains(&source) {
                return (400, format!("'{}' is not a valid collection name. Please enter 'LIST COLLECTIONS' to obtain a list of valid collections.", source), vec![]);
            }

            let mut final_key_indexes = Vec::<usize>::new();
            let target_all = keys.len() == 1 && keys[0] == "*";

            if source == "users" {
                let all_users = match auto_fetch_all_users(&all_mappings) {
                    Ok(users) => users,
                    Err(e) => {
                        return (500, format!("Error while fetching users: {}", e), vec![]);
                    }
                };

                let properties = User::obtain_properties();
                let mut users_vec: Vec<String> = vec![];

                obtain_indexes(
                    keys,
                    &mut final_key_indexes,
                    ";",
                    target_all,
                    &properties,
                    &mut users_vec,
                );

                let all_users_str = stringify_users(&all_users);
                perform_selection(
                    all_users_str,
                    target_all,
                    final_key_indexes,
                    ";",
                    &mut users_vec,
                    &properties,
                    &and_arguments,
                    &or_arguments,
                );

                processed_queries.push(users_vec);
            } else if source == "configs" {
                let all_configs = match auto_fetch_all_configs(&all_mappings) {
                    Ok(configs) => configs,
                    Err(e) => {
                        return (500, format!("Error while fetching configs: {}", e), vec![]);
                    }
                };

                let properties = Config::obtain_properties();
                let mut configs_vec: Vec<String> = vec![];

                obtain_indexes(
                    keys,
                    &mut final_key_indexes,
                    "|",
                    target_all,
                    &properties,
                    &mut configs_vec,
                );

                let all_configs_str = stringify_configs(&all_configs);
                perform_selection(
                    all_configs_str,
                    target_all,
                    final_key_indexes,
                    "|",
                    &mut configs_vec,
                    &properties,
                    &and_arguments,
                    &or_arguments,
                );

                processed_queries.push(configs_vec);
            } else if source == "events" {
                let all_events = match auto_fetch_all_events(&all_mappings) {
                    Ok(events) => events,
                    Err(e) => {
                        return (500, format!("Error while fetching events: {}", e), vec![]);
                    }
                };

                let properties = Event::obtain_properties();
                let mut events_vec: Vec<String> = vec![];

                obtain_indexes(
                    keys,
                    &mut final_key_indexes,
                    ";",
                    target_all,
                    &properties,
                    &mut events_vec,
                );

                let all_events_str = stringify_events(&all_events);
                perform_selection(
                    all_events_str,
                    target_all,
                    final_key_indexes,
                    ";",
                    &mut events_vec,
                    &properties,
                    &and_arguments,
                    &or_arguments,
                );

                processed_queries.push(events_vec);
            } else if source == "mappings" {
                let properties = Mapping::obtain_properties();
                let mut mappings_vec: Vec<String> = vec![];

                obtain_indexes(
                    keys,
                    &mut final_key_indexes,
                    "=",
                    target_all,
                    &properties,
                    &mut mappings_vec,
                );

                let all_mappings_str = stringify_mappings(&all_mappings);
                perform_selection(
                    all_mappings_str,
                    target_all,
                    final_key_indexes,
                    "=",
                    &mut mappings_vec,
                    &properties,
                    &and_arguments,
                    &or_arguments,
                );

                processed_queries.push(mappings_vec);
            } else if source == "medias" {
                let all_medias = match auto_fetch_all_medias(&all_mappings) {
                    Ok(medias) => medias,
                    Err(e) => {
                        return (500, format!("Error while fetching medias: {}", e), vec![]);
                    }
                };

                let properties = Media::obtain_properties();
                let mut medias_vec: Vec<String> = vec![];

                obtain_indexes(
                    keys,
                    &mut final_key_indexes,
                    "^",
                    target_all,
                    &properties,
                    &mut medias_vec,
                );

                let all_medias_str = stringify_medias(&all_medias);
                perform_selection(
                    all_medias_str,
                    target_all,
                    final_key_indexes,
                    "^",
                    &mut medias_vec,
                    &properties,
                    &and_arguments,
                    &or_arguments,
                );

                processed_queries.push(medias_vec);
            } else if source == "projects" {
                let all_projects = match auto_fetch_all_projects(&all_mappings) {
                    Ok(medias) => medias,
                    Err(e) => {
                        return (500, format!("Error while fetching projects: {}", e), vec![]);
                    }
                };

                let properties = Project::obtain_properties();
                let mut projects_vec: Vec<String> = vec![];

                obtain_indexes(
                    keys,
                    &mut final_key_indexes,
                    ";",
                    target_all,
                    &properties,
                    &mut projects_vec,
                );

                let all_projects_str = stringify_projects(&all_projects);
                perform_selection(
                    all_projects_str,
                    target_all,
                    final_key_indexes,
                    ";",
                    &mut projects_vec,
                    &properties,
                    &and_arguments,
                    &or_arguments,
                );

                processed_queries.push(projects_vec);
            } else if source == "constraints" {
                let all_constraints = match auto_fetch_all_constraints(&all_mappings) {
                    Ok(constraints) => constraints,
                    Err(e) => {
                        return (
                            500,
                            format!("Error while fetching constraints: {}", e),
                            vec![],
                        );
                    }
                };

                let mut properties = Constraint::obtain_properties();
                properties = properties.replace(";", "??");
                let mut constraints_vec: Vec<String> = vec![];

                obtain_indexes(
                    keys,
                    &mut final_key_indexes,
                    "??",
                    target_all,
                    &properties,
                    &mut constraints_vec,
                );

                let mut all_constraints_str = String::new();
                for constraint in all_constraints {
                    if constraint.properties.len() == 0 {
                        all_constraints_str = format!(
                            "{}{}{}??????????????",
                            all_constraints_str,
                            if all_constraints_str.chars().count() > 1 {
                                "\n"
                            } else {
                                ""
                            },
                            constraint.component_name
                        );
                    }

                    for constraint_property in constraint.properties {
                        let mut property_str = ConstraintProperty::to_string(constraint_property);
                        property_str = property_str.replacen(";", "??", 4);
                        property_str = property_str.replace(";not_allowed=", "??");
                        property_str = property_str.replace(";allowed=", "??");
                        all_constraints_str = format!(
                            "{}{}{}??{}",
                            all_constraints_str,
                            if all_constraints_str.chars().count() > 1 {
                                "\n"
                            } else {
                                ""
                            },
                            constraint.component_name,
                            property_str
                        );
                    }
                }

                perform_selection(
                    all_constraints_str,
                    target_all,
                    final_key_indexes,
                    "??",
                    &mut constraints_vec,
                    &properties,
                    &and_arguments,
                    &or_arguments,
                );

                processed_queries.push(constraints_vec);
            }
        } else if current_query[0] == "list" {
            if current_query[1] == "collections" {
                let collections_list = vec![
                    String::from("collections (UPCOMING)"),
                    String::from("configs"),
                    String::from("constraints"),
                    String::from("data (UPCOMING)"),
                    String::from("events"),
                    String::from("mappings"),
                    String::from("medias"),
                    String::from("projects"),
                    String::from("routes (UPCOMING)"),
                    String::from("users"),
                ];

                return (200, format!("Successful Query!"), vec![collections_list]);
            }
            return (400, invalid_query_msg, vec![]);
        } else if current_query[0] == "help" {
            let mut help_list = vec![];
            help_list.push(vec![String::from("Commands:")]);

            help_list.push(vec![
                    String::from("help"),
                    String::from("Obtain help on possible commands that can be executed along with examples and explanations."),
                ]);

            help_list.push(vec![
                String::from("LIST COLLECTIONS;"),
                String::from("Obtain a list of all the possible collections on which queries can be executed."),
            ]);

            help_list.push(vec![
                String::from("SELECT [KEYS | *] FROM COLLECTION_NAME [WHERE CONDITIONS [AND_ARGUMENTS] [OR_ARGUMENTS] ];"),
                String::from("Obtain desired data from a valid collection that can be tweaked using conditions."),
                String::from("KEYS correspond to the properties that should be returned from matching records in the specified collection. An asterisk (*) can be used to return all properties."),
                String::from("The WHERE clause is optional and can be used to filter the records that are returned."),
                String::from("AND_ARGUMENTS and/or OR_ARGUMENTS can be used to further filter the records that are returned. If both are used, a record matching **any** OR_ARGUMENT will be returned, else it will need to match **every** AND_ARGUMENT specified in order to get returned."),
                String::from("Example 1: SELECT * FROM users;"),
                String::from("Example 2: SELECT property_name, component_name, max FROM constraints WHERE is_alphabetic == \"false\";"),
                String::from("Example 3: SELECT id, description FROM events WHERE event_type == \"project_update\" AND 2 > 1 OR redirect == event_type;"),
            ]);

            return (200, format!("Successful Query!"), help_list);
        } else {
            return (400, invalid_query_msg, vec![]);
        }
    }

    // collection
    // custom_structure
    // data
    // datapair
    // structure
    // routes

    if processed_queries.len() <= 0 {
        return (400, invalid_query_msg, vec![]);
    }

    (200, format!("Successful Queries!"), processed_queries)
}

fn obtain_indexes(
    keys: Vec<String>,
    final_key_indexes: &mut Vec<usize>,
    delimiter: &str,
    target_all: bool,
    properties: &String,
    final_vec: &mut Vec<String>,
) {
    let mut title_vec = Vec::<String>::new();
    let tmp_vec = properties.split(delimiter);
    for key in keys {
        for (i, tmp) in tmp_vec.clone().enumerate() {
            if tmp == key {
                final_key_indexes.push(i);
                title_vec.push(key.clone());
                break;
            }
        }
    }

    if target_all {
        final_vec.push(properties.clone());
    } else {
        final_vec.push(title_vec.join(delimiter));
    }
}

fn perform_selection(
    main_string: String,
    target_all: bool,
    final_key_indexes: Vec<usize>,
    delimiter: &str,
    final_vec: &mut Vec<String>,
    properties: &String,
    and_arguments: &Vec<(&str, &str, &str)>,
    or_arguments: &Vec<(&str, &str, &str)>,
) {
    for str in main_string.split("\n") {
        if !decide_to_keep(and_arguments, or_arguments, properties, str, delimiter) {
            continue;
        }

        if target_all {
            final_vec.push(str.to_string());
            continue;
        }

        let mut tmp_str = String::new();

        for (i, index) in final_key_indexes.iter().enumerate() {
            let tmp_vec = str.split(delimiter).collect::<Vec<&str>>();
            tmp_str.push_str(tmp_vec[*index]);
            if i != final_key_indexes.len() - 1 {
                tmp_str.push_str(delimiter);
            }
        }
        final_vec.push(tmp_str.to_string());
    }
}

fn decide_to_keep(
    and_arguments: &Vec<(&str, &str, &str)>,
    or_arguments: &Vec<(&str, &str, &str)>,
    properties: &String,
    value_str: &str,
    delimiter: &str,
) -> bool {
    let properties_vec = properties.split(delimiter);
    let value_vec = value_str.split(delimiter).collect::<Vec<&str>>();

    for or_arg in or_arguments {
        let mut left = or_arg.0.to_string();
        let operator = or_arg.1;
        let mut right = or_arg.2.to_string();

        if left.len() <= 0 || right.len() <= 0 {
            continue;
        }

        let left_num = process_operand(&mut left, &properties_vec, &value_vec);
        let right_num = process_operand(&mut right, &properties_vec, &value_vec);

        if perform_comparison(&left, &right, operator, left_num, right_num) {
            return true;
        }
    }

    for and_arg in and_arguments {
        let mut left = and_arg.0.to_string();
        let operator = and_arg.1;
        let mut right = and_arg.2.to_string();

        if left.len() <= 0 || right.len() <= 0 {
            continue;
        }

        let left_num = process_operand(&mut left, &properties_vec, &value_vec);
        let right_num = process_operand(&mut right, &properties_vec, &value_vec);

        if !perform_comparison(&left, &right, operator, left_num, right_num) {
            return false;
        }
    }

    true
}

fn process_operand(
    value: &mut String,
    properties_vec: &Split<&str>,
    value_vec: &Vec<&str>,
) -> (bool, bool) {
    if value.len() <= 0 {
        return (false, false);
    }

    if value.contains("'") || value.contains("\"") {
        *value = value.replace("'", "");
        *value = value.replace("\"", "");
    } else if value
        .chars()
        .nth(0)
        .unwrap()
        .to_string()
        .parse::<usize>()
        .is_ok()
    {
        if value.contains(".") {
            return (false, true);
        } else {
            return (true, false);
        }
    } else {
        for (i, property) in properties_vec.clone().enumerate() {
            if property == value {
                *value = value_vec[i].to_string();
                break;
            }
        }
    }
    return (false, false);
}

fn perform_comparison(
    left: &String,
    right: &String,
    operator: &str,
    left_num: (bool, bool),
    right_num: (bool, bool),
) -> bool {
    if left_num.0 && right_num.0 {
        let left_int = left.parse::<i32>().unwrap();
        let right_int = right.parse::<i32>().unwrap();

        match operator {
            "==" => return left_int == right_int,
            "!=" => return left_int != right_int,
            ">" => return left_int > right_int,
            ">=" => return left_int >= right_int,
            "<" => return left_int < right_int,
            "<=" => return left_int <= right_int,
            _ => return false,
        }
    } else if left_num.1 && right_num.1 {
        let left_float = left.parse::<f32>().unwrap();
        let right_float = right.parse::<f32>().unwrap();

        match operator {
            "==" => return left_float == right_float,
            "!=" => return left_float != right_float,
            ">" => return left_float > right_float,
            ">=" => return left_float >= right_float,
            "<" => return left_float < right_float,
            "<=" => return left_float <= right_float,
            _ => return false,
        }
    } else {
        match operator {
            "==" => return left == right,
            "!=" => return left != right,
            _ => return false,
        }
    }
}
