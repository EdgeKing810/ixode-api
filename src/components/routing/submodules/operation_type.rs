#![allow(non_camel_case_types)]

use std::fmt;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationType {
    GREATER_THAN,
    LESS_THAN,
    EQUAL_TO,
    NOT_EQUAL_TO,
    GREATER_THAN_OR_EQUAL_TO,
    LESS_THAN_OR_EQUAL_TO,
    ADDITION,
    SUBSTRACTION,
    MULTIPLICATION,
    DIVISION,
    MODULAR,
    INCLUDES,
    NONE,
}

impl Default for OperationType {
    fn default() -> Self {
        OperationType::EQUAL_TO
    }
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let otype_txt = match self {
            OperationType::GREATER_THAN => "GREATER_THAN".to_string(),
            OperationType::LESS_THAN => "LESS_THAN".to_string(),
            OperationType::EQUAL_TO => "EQUAL_TO".to_string(),
            OperationType::NOT_EQUAL_TO => "NOT_EQUAL_TO".to_string(),
            OperationType::GREATER_THAN_OR_EQUAL_TO => "GREATER_THAN_OR_EQUAL_TO".to_string(),
            OperationType::LESS_THAN_OR_EQUAL_TO => "LESS_THAN_OR_EQUAL_TO".to_string(),
            OperationType::ADDITION => "ADDITION".to_string(),
            OperationType::SUBSTRACTION => "SUBSTRACTION".to_string(),
            OperationType::MULTIPLICATION => "MULTIPLICATION".to_string(),
            OperationType::DIVISION => "DIVISION".to_string(),
            OperationType::MODULAR => "MODULAR".to_string(),
            OperationType::INCLUDES => "INCLUDES".to_string(),
            OperationType::NONE => "NONE".to_string(),
        };

        write!(f, "{}", otype_txt)
    }
}

impl OperationType {
    pub fn to(otype: OperationType) -> String {
        return match otype.clone() {
            OperationType::GREATER_THAN => "GREATER_THAN".to_string(),
            OperationType::LESS_THAN => "LESS_THAN".to_string(),
            OperationType::EQUAL_TO => "EQUAL_TO".to_string(),
            OperationType::NOT_EQUAL_TO => "NOT_EQUAL_TO".to_string(),
            OperationType::GREATER_THAN_OR_EQUAL_TO => "GREATER_THAN_OR_EQUAL_TO".to_string(),
            OperationType::LESS_THAN_OR_EQUAL_TO => "LESS_THAN_OR_EQUAL_TO".to_string(),
            OperationType::ADDITION => "ADDITION".to_string(),
            OperationType::SUBSTRACTION => "SUBSTRACTION".to_string(),
            OperationType::MULTIPLICATION => "MULTIPLICATION".to_string(),
            OperationType::DIVISION => "DIVISION".to_string(),
            OperationType::MODULAR => "MODULAR".to_string(),
            OperationType::INCLUDES => "INCLUDES".to_string(),
            OperationType::NONE => "NONE".to_string(),
        };
    }

    pub fn from(otype_txt: &str) -> OperationType {
        return match otype_txt.to_uppercase().as_str() {
            "GREATER_THAN" => OperationType::GREATER_THAN,
            "LESS_THAN" => OperationType::LESS_THAN,
            "EQUAL_TO" => OperationType::EQUAL_TO,
            "NOT_EQUAL_TO" => OperationType::NOT_EQUAL_TO,
            "GREATER_THAN_OR_EQUAL_TO" => OperationType::GREATER_THAN_OR_EQUAL_TO,
            "LESS_THAN_OR_EQUAL_TO" => OperationType::LESS_THAN_OR_EQUAL_TO,
            "ADDITION" => OperationType::ADDITION,
            "SUBSTRACTION" => OperationType::SUBSTRACTION,
            "MULTIPLICATION" => OperationType::MULTIPLICATION,
            "DIVISION" => OperationType::DIVISION,
            "MODULAR" => OperationType::MODULAR,
            "INCLUDES" => OperationType::INCLUDES,
            "NONE" => OperationType::NONE,
            _ => OperationType::EQUAL_TO,
        };
    }
}
