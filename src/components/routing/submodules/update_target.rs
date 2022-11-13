use rocket::serde::{Deserialize, Serialize};

use crate::{
    components::{
        constraint_property::ConstraintProperty,
        routing::submodules::sub_condition_plain::ConditionPlain,
    },
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateTarget {
    pub field: String,
    pub conditions: Vec<ConditionPlain>,
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

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "update_target", "field", field) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for n in 0..all_targets.len() {
            if n as u32 == index {
                found_target = Some(all_targets[index as usize].clone());
                all_targets[index as usize].field = final_value;
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
        new_condition: ConditionPlain,
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

                let mut updated_conditions = Vec::<ConditionPlain>::new();
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
        conditions: Vec<ConditionPlain>,
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

        let mut all_conditions: Vec<ConditionPlain> = Vec::new();
        let conditions_list = conditions_list_str.trim().split(">").collect::<Vec<&str>>();

        for c_str in conditions_list {
            if c_str.len() < 1 {
                continue;
            }

            if let Err(e) = ConditionPlain::from_string(&mut all_conditions, c_str) {
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
        let conditions_str = ConditionPlain::stringify(&target.conditions);

        format!("{{{}|{}}}", target.field, conditions_str)
    }
}
