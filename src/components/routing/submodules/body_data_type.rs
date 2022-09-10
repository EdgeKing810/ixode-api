use std::fmt;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BodyDataType {
    INTEGER,
    STRING,
    BOOLEAN,
    OTHER,
}

impl Default for BodyDataType {
    fn default() -> Self {
        BodyDataType::STRING
    }
}

impl fmt::Display for BodyDataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bdtype_txt = match self {
            BodyDataType::INTEGER => "INTEGER",
            BodyDataType::STRING => "STRING",
            BodyDataType::BOOLEAN => "BOOLEAN",
            BodyDataType::OTHER => "OTHER",
        };

        write!(f, "{}", bdtype_txt)
    }
}

impl BodyDataType {
    pub fn to(bdtype: BodyDataType) -> String {
        return match bdtype.clone() {
            BodyDataType::INTEGER => "INTEGER".to_string(),
            BodyDataType::STRING => "STRING".to_string(),
            BodyDataType::BOOLEAN => "BOOLEAN".to_string(),
            BodyDataType::OTHER => "OTHER".to_string(),
        };
    }

    pub fn from(bdtype_txt: &str) -> BodyDataType {
        return match bdtype_txt.to_uppercase().as_str() {
            "INTEGER" => BodyDataType::INTEGER,
            "STRING" => BodyDataType::STRING,
            "BOOLEAN" => BodyDataType::BOOLEAN,
            "OTHER" => BodyDataType::OTHER,
            _ => BodyDataType::OTHER,
        };
    }
}
