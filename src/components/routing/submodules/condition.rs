use super::{
    sub_condition_type::ConditionType, sub_next_condition_type::NextConditionType,
    sub_ref_data::RefData,
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Condition {
    pub left: RefData,
    pub right: RefData,
    pub condition_type: ConditionType,
    pub not: bool,
    pub next: NextConditionType,
}

impl Condition {
    pub fn create(
        all_conditions: &mut Vec<Condition>,
        left: RefData,
        right: RefData,
        condition_type: &str,
        not: bool,
        next: &str,
    ) {
        let new_condition = Condition {
            left: left.clone(),
            right: right.clone(),
            condition_type: ConditionType::from(condition_type),
            not: not,
            next: NextConditionType::from(next),
        };
        all_conditions.push(new_condition);
    }

    pub fn stringify(all_conditions: &Vec<Condition>) -> String {
        let mut stringified_conditions = String::new();

        for condition in all_conditions {
            stringified_conditions = format!(
                "{}{}{}",
                stringified_conditions,
                if stringified_conditions.chars().count() > 1 {
                    ">"
                } else {
                    ""
                },
                Condition::to_string(condition.clone()),
            );
        }

        stringified_conditions
    }

    pub fn from_string(
        all_conditions: &mut Vec<Condition>,
        condition_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_condition = condition_str.split("(").collect::<Vec<&str>>();
        if current_condition.len() <= 1 {
            return Err((
                500,
                String::from("Invalid condition (at declaration start)"),
            ));
        }

        current_condition = current_condition[1].split(")").collect::<Vec<&str>>();
        if current_condition.len() <= 1 {
            return Err((500, String::from("Invalid condition (at declaration end)")));
        }

        current_condition = current_condition[0].split("|").collect::<Vec<&str>>();
        if current_condition.len() < 5 {
            return Err((500, String::from("Invalid condition (in format)")));
        }

        let not_str = current_condition[3].split("not=").collect::<Vec<&str>>();
        if not_str.len() <= 1 {
            return Err((500, String::from("Invalid condition (in 'not' format)")));
        }

        let not = not_str[1] == "true";

        let next_str = current_condition[4].split("next=").collect::<Vec<&str>>();
        if next_str.len() <= 1 {
            return Err((500, String::from("Invalid condition (in 'next' format)")));
        }

        let left = match RefData::from_string(current_condition[0]) {
            Ok(left) => left,
            Err(err) => {
                return Err((
                    500,
                    format!("Invalid condition (in 'left' format) -> {}", err.1),
                ))
            }
        };

        let right = match RefData::from_string(current_condition[2]) {
            Ok(right) => right,
            Err(err) => {
                return Err((
                    500,
                    format!("Invalid condition (in 'right' format) -> {}", err.1),
                ))
            }
        };

        Condition::create(
            all_conditions,
            left,
            right,
            current_condition[1],
            not,
            next_str[1],
        );

        Ok(())
    }

    pub fn to_string(condition: Condition) -> String {
        format!(
            "({}|{}|{}|not={}|next={})",
            RefData::to_string(condition.left.clone()),
            ConditionType::to(condition.condition_type.clone()),
            RefData::to_string(condition.right.clone()),
            if condition.not == true {
                "true"
            } else {
                "false"
            },
            NextConditionType::to(condition.next.clone())
        )
    }
}
