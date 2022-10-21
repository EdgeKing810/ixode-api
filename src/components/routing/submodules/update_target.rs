use rocket::serde::{Deserialize, Serialize};

use crate::components::routing::submodules::sub_filter::Filter;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateTarget {
    pub field: String,
    pub conditions: Vec<Filter>,
}

impl UpdateTarget {
    pub fn create(all_targets: &mut Vec<UpdateTarget>, field: &str) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_target = UpdateTarget {
            field: "".to_string(),
            conditions: vec![],
        };
        all_targets.push(new_target);

        let index = (all_targets.len() - 1) as u32;

        if !has_error {
            let field_update = Self::update_field(all_targets, index, field);
            if let Err(e) = field_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_target = Self::delete(all_targets, index);
            if let Err(e) = delete_target {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_targets: &Vec<UpdateTarget>, index: u32) -> bool {
        let mut found = false;

        for n in 0..all_targets.len() {
            if n as u32 == index {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_field(
        all_targets: &mut Vec<UpdateTarget>,
        index: u32,
        field: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_target: Option<UpdateTarget> = None;

        if !String::from(field)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: field contains an invalid character"),
            ));
        }

        if String::from(field.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: field does not contain enough characters"),
            ));
        } else if String::from(field.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: field contains too many characters"),
            ));
        }

        for n in 0..all_targets.len() {
            if n as u32 == index {
                found_target = Some(all_targets[index as usize].clone());
                all_targets[index as usize].field = field.to_string();
                break;
            }
        }

        if let None = found_target {
            return Err((404, String::from("Error: Target not found")));
        }

        Ok(())
    }

    pub fn add_condition(
        all_targets: &mut Vec<UpdateTarget>,
        index: u32,
        new_condition: Filter,
    ) -> Result<(), (usize, String)> {
        let mut found_target: Option<UpdateTarget> = None;

        for n in 0..all_targets.len() {
            if n as u32 == index {
                found_target = Some(all_targets[index as usize].clone());
                all_targets[index as usize].conditions.push(new_condition);
                break;
            }
        }

        if let None = found_target {
            return Err((404, String::from("Error: Target not found")));
        }

        Ok(())
    }

    pub fn remove_condition(
        all_targets: &mut Vec<UpdateTarget>,
        index: u32,
        condition_index: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_target: Option<UpdateTarget> = None;

        for n in 0..all_targets.len() {
            if n as u32 == index {
                found_target = Some(all_targets[index as usize].clone());

                let mut updated_conditions = Vec::<Filter>::new();
                if condition_index >= all_targets[index as usize].conditions.len() as u32 {
                    return Err((
                        400,
                        String::from("Error: Index goes over the amount of conditions present"),
                    ));
                }

                for i in 0..all_targets[index as usize].conditions.len() {
                    if i as u32 != condition_index {
                        updated_conditions.push(all_targets[index as usize].conditions[i].clone());
                    }
                }

                all_targets[index as usize].conditions = updated_conditions;
                break;
            }
        }

        if let None = found_target {
            return Err((404, String::from("Error: Target not found")));
        }

        Ok(())
    }

    pub fn set_conditions(
        all_targets: &mut Vec<UpdateTarget>,
        index: u32,
        conditions: Vec<Filter>,
    ) -> Result<(), (usize, String)> {
        let mut found_target: Option<UpdateTarget> = None;

        for n in 0..all_targets.len() {
            if n as u32 == index {
                found_target = Some(all_targets[index as usize].clone());
                all_targets[index as usize].conditions = conditions;
                break;
            }
        }

        if let None = found_target {
            return Err((404, String::from("Error: Target not found")));
        }

        Ok(())
    }

    pub fn delete(all_targets: &mut Vec<UpdateTarget>, index: u32) -> Result<(), (usize, String)> {
        let mut found_target: Option<UpdateTarget> = None;

        for n in 0..all_targets.len() {
            if n as u32 == index {
                found_target = Some(all_targets[index as usize].clone());
                break;
            }
        }

        if let None = found_target {
            return Err((404, String::from("Error: Target not found")));
        }

        let mut updated_targets = Vec::<UpdateTarget>::new();

        for n in 0..all_targets.len() {
            if n as u32 != index {
                updated_targets.push(all_targets[index as usize].clone());
            }
        }

        *all_targets = updated_targets;

        Ok(())
    }

    pub fn stringify(all_targets: &Vec<UpdateTarget>) -> String {
        let mut stringified_targets = String::new();

        for target in all_targets {
            stringified_targets = format!(
                "{}{}{}",
                stringified_targets,
                if stringified_targets.chars().count() > 1 {
                    "%"
                } else {
                    ""
                },
                UpdateTarget::to_string(target.clone()),
            );
        }

        stringified_targets
    }

    pub fn from_string(
        all_targets: &mut Vec<UpdateTarget>,
        target_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_target = target_str.split("{").collect::<Vec<&str>>();
        if current_target.len() <= 1 {
            return Err((500, String::from("Invalid target (at declaration start)")));
        }

        current_target = current_target[1].split("}").collect::<Vec<&str>>();
        if current_target.len() <= 1 {
            return Err((500, String::from("Invalid target (at declaration end)")));
        }

        current_target = current_target[0].split("|").collect::<Vec<&str>>();
        if current_target.len() < 2 {
            return Err((500, String::from("Invalid target (in format)")));
        }

        let field = current_target[0];

        let conditions_list_str = current_target[1..].join("|");

        let mut all_conditions: Vec<Filter> = Vec::new();
        let conditions_list = conditions_list_str.trim().split(">").collect::<Vec<&str>>();

        for c_str in conditions_list {
            if c_str.len() < 1 {
                continue;
            }

            if let Err(e) = Filter::from_string(&mut all_conditions, c_str) {
                return Err((500, format!("Invalid condition in target -> {}", e.1)));
            };
        }

        if let Err(e) = UpdateTarget::create(all_targets, field) {
            return Err((500, format!("Invalid target (while processing) -> {}", e.1)));
        };

        match UpdateTarget::set_conditions(
            all_targets,
            (all_targets.len() - 1) as u32,
            all_conditions,
        ) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err((500, format!("Invalid target (while processing) -> {}", e.1)));
            }
        };
    }

    pub fn to_string(target: UpdateTarget) -> String {
        let conditions_str = Filter::stringify(&target.conditions);

        format!("{{{}|{}}}", target.field, conditions_str)
    }
}
