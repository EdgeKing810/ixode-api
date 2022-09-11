use super::{
    sub_next_condition_type::NextConditionType, sub_operation_type::OperationType,
    sub_ref_data::RefData,
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub left: RefData,
    pub right: RefData,
    pub operation_type: OperationType,
    pub not: bool,
    pub next: NextConditionType,
}

impl Operation {
    pub fn create(
        all_operations: &mut Vec<Operation>,
        left: RefData,
        right: RefData,
        operation_type: &str,
        not: bool,
        next: &str,
    ) {
        let new_operation = Operation {
            left: left.clone(),
            right: right.clone(),
            operation_type: OperationType::from(operation_type),
            not: not,
            next: NextConditionType::from(next),
        };
        all_operations.push(new_operation);
    }

    pub fn stringify(all_operations: &Vec<Operation>) -> String {
        let mut stringified_operations = String::new();

        for operation in all_operations {
            stringified_operations = format!(
                "{}{}{}",
                stringified_operations,
                if stringified_operations.chars().count() > 1 {
                    ">"
                } else {
                    ""
                },
                Operation::to_string(operation.clone()),
            );
        }

        stringified_operations
    }

    pub fn from_string(
        all_operations: &mut Vec<Operation>,
        operation_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_operation = operation_str.split("(").collect::<Vec<&str>>();
        if current_operation.len() <= 1 {
            return Err((500, String::from("Error: Invalid operation string / 1")));
        }

        current_operation = current_operation[1].split(")").collect::<Vec<&str>>();
        if current_operation.len() <= 1 {
            return Err((500, String::from("Error: Invalid operation string / 2")));
        }

        current_operation = current_operation[0].split("|").collect::<Vec<&str>>();
        if current_operation.len() < 5 {
            return Err((500, String::from("Error: Invalid condition string / 3")));
        }

        let not_str = current_operation[3].split("not=").collect::<Vec<&str>>();
        if not_str.len() <= 1 {
            return Err((500, String::from("Error: Invalid condition string / 4")));
        }

        let not = not_str[1] == "true";

        let next_str = current_operation[4].split("next=").collect::<Vec<&str>>();
        if next_str.len() <= 1 {
            return Err((500, String::from("Error: Invalid operation string / 5")));
        }

        let left = match RefData::from_string(current_operation[0]) {
            Ok(left) => left,
            Err(err) => return Err(err),
        };

        let right = match RefData::from_string(current_operation[2]) {
            Ok(right) => right,
            Err(err) => return Err(err),
        };

        Operation::create(
            all_operations,
            left,
            right,
            current_operation[1],
            not,
            next_str[1],
        );

        Ok(())
    }

    pub fn to_string(operation: Operation) -> String {
        format!(
            "({}|{}|{}|not={}|next={})",
            RefData::to_string(operation.left.clone()),
            OperationType::to(operation.operation_type.clone()),
            RefData::to_string(operation.right.clone()),
            if operation.not == true {
                "true"
            } else {
                "false"
            },
            NextConditionType::to(operation.next.clone())
        )
    }
}
