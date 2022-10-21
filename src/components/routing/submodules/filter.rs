use super::{
    sub_next_condition_type::NextConditionType, sub_operation_type::OperationType,
    sub_ref_data::RefData,
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Filter {
    pub right: RefData,
    pub operation_type: OperationType,
    pub not: bool,
    pub next: NextConditionType,
}

impl Filter {
    pub fn create(
        all_filters: &mut Vec<Filter>,
        right: RefData,
        operation_type: &str,
        not: bool,
        next: &str,
    ) {
        let new_filter = Filter {
            right: right.clone(),
            operation_type: OperationType::from(operation_type),
            not: not,
            next: NextConditionType::from(next),
        };
        all_filters.push(new_filter);
    }

    pub fn stringify(all_filters: &Vec<Filter>) -> String {
        let mut stringified_filters = String::new();

        for filter in all_filters {
            stringified_filters = format!(
                "{}{}{}",
                stringified_filters,
                if stringified_filters.chars().count() > 1 {
                    ">"
                } else {
                    ""
                },
                Filter::to_string(filter.clone()),
            );
        }

        stringified_filters
    }

    pub fn from_string(
        all_filters: &mut Vec<Filter>,
        filter_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_filter = filter_str.split("(").collect::<Vec<&str>>();
        if current_filter.len() <= 1 {
            return Err((500, String::from("Invalid filter (at declaration start)")));
        }

        current_filter = current_filter[1].split(")").collect::<Vec<&str>>();
        if current_filter.len() <= 1 {
            return Err((500, String::from("Invalid filter (at declaration end)")));
        }

        current_filter = current_filter[0].split("|").collect::<Vec<&str>>();
        if current_filter.len() < 4 {
            return Err((500, String::from("Invalid filter (in format)")));
        }

        let not_str = current_filter[2].split("not=").collect::<Vec<&str>>();
        if not_str.len() <= 1 {
            return Err((500, String::from("Invalid filter (in 'not' format)")));
        }

        let not = not_str[1] == "true";

        let next_str = current_filter[3].split("next=").collect::<Vec<&str>>();
        if next_str.len() <= 1 {
            return Err((500, String::from("Invalid filter (in 'next' format)")));
        }

        let right = match RefData::from_string(current_filter[0]) {
            Ok(right) => right,
            Err(err) => {
                return Err((
                    500,
                    format!("Invalid filter (in 'right' format) -> {}", err.1),
                ))
            }
        };

        Filter::create(all_filters, right, current_filter[1], not, next_str[1]);

        Ok(())
    }

    pub fn to_string(filter: Filter) -> String {
        format!(
            "({}|{}|not={}|next={})",
            RefData::to_string(filter.right.clone()),
            OperationType::to(filter.operation_type.clone()),
            if filter.not == true { "true" } else { "false" },
            NextConditionType::to(filter.next.clone())
        )
    }
}
