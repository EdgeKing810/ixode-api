#![allow(non_camel_case_types)]

use std::fmt;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    GREATER_THAN,
    LESS_THAN,
    EQUAL_TO,
    NOT_EQUAL_TO,
    GREATER_THAN_OR_EQUAL_TO,
    LESS_THAN_OR_EQUAL_TO,
    INCLUDES,
}

impl Default for ConditionType {
    fn default() -> Self {
        ConditionType::EQUAL_TO
    }
}

impl fmt::Display for ConditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ctype_txt = match self {
            ConditionType::GREATER_THAN => "GREATER_THAN".to_string(),
            ConditionType::LESS_THAN => "LESS_THAN".to_string(),
            ConditionType::EQUAL_TO => "EQUAL_TO".to_string(),
            ConditionType::NOT_EQUAL_TO => "NOT_EQUAL_TO".to_string(),
            ConditionType::GREATER_THAN_OR_EQUAL_TO => "GREATER_THAN_OR_EQUAL_TO".to_string(),
            ConditionType::LESS_THAN_OR_EQUAL_TO => "LESS_THAN_OR_EQUAL_TO".to_string(),
            ConditionType::INCLUDES => "INCLUDES".to_string(),
        };

        write!(f, "{}", ctype_txt)
    }
}

impl ConditionType {
    pub fn to(ctype: ConditionType) -> String {
        return match ctype.clone() {
            ConditionType::GREATER_THAN => "GREATER_THAN".to_string(),
            ConditionType::LESS_THAN => "LESS_THAN".to_string(),
            ConditionType::EQUAL_TO => "EQUAL_TO".to_string(),
            ConditionType::NOT_EQUAL_TO => "NOT_EQUAL_TO".to_string(),
            ConditionType::GREATER_THAN_OR_EQUAL_TO => "GREATER_THAN_OR_EQUAL_TO".to_string(),
            ConditionType::LESS_THAN_OR_EQUAL_TO => "LESS_THAN_OR_EQUAL_TO".to_string(),
            ConditionType::INCLUDES => "INCLUDES".to_string(),
        };
    }

    pub fn from(ctype_txt: &str) -> ConditionType {
        return match ctype_txt.to_uppercase().as_str() {
            "GREATER_THAN" => ConditionType::GREATER_THAN,
            "LESS_THAN" => ConditionType::LESS_THAN,
            "EQUAL_TO" => ConditionType::EQUAL_TO,
            "NOT_EQUAL_TO" => ConditionType::NOT_EQUAL_TO,
            "GREATER_THAN_OR_EQUAL_TO" => ConditionType::GREATER_THAN_OR_EQUAL_TO,
            "LESS_THAN_OR_EQUAL_TO" => ConditionType::LESS_THAN_OR_EQUAL_TO,
            "INCLUDES" => ConditionType::INCLUDES,
            _ => ConditionType::EQUAL_TO,
        };
    }
}
