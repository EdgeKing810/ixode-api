use std::fmt;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NextConditionType {
    AND,
    OR,
    NONE,
}

impl Default for NextConditionType {
    fn default() -> Self {
        NextConditionType::NONE
    }
}

impl fmt::Display for NextConditionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let nctype_txt = match self {
            NextConditionType::AND => "AND",
            NextConditionType::OR => "OR",
            NextConditionType::NONE => "NONE",
        };

        write!(f, "{}", nctype_txt)
    }
}

impl NextConditionType {
    pub fn to(nctype: NextConditionType) -> String {
        return match nctype.clone() {
            NextConditionType::AND => "AND".to_string(),
            NextConditionType::OR => "OR".to_string(),
            NextConditionType::NONE => "NONE".to_string(),
        };
    }

    pub fn from(nctype_txt: &str) -> NextConditionType {
        return match nctype_txt.to_uppercase().as_str() {
            "AND" => NextConditionType::AND,
            "OR" => NextConditionType::OR,
            "NONE" => NextConditionType::NONE,
            _ => NextConditionType::NONE,
        };
    }
}
