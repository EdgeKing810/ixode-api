use std::fmt;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionAction {
    FAIL,
    BREAK,
    CONTINUE,
}

impl Default for ConditionAction {
    fn default() -> Self {
        ConditionAction::CONTINUE
    }
}

impl fmt::Display for ConditionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let caction_txt = match self {
            ConditionAction::FAIL => "FAIL",
            ConditionAction::BREAK => "BREAK",
            ConditionAction::CONTINUE => "CONTINUE",
        };

        write!(f, "{}", caction_txt)
    }
}

impl ConditionAction {
    pub fn to(caction: ConditionAction) -> String {
        return match caction.clone() {
            ConditionAction::FAIL => "FAIL".to_string(),
            ConditionAction::BREAK => "BREAK".to_string(),
            ConditionAction::CONTINUE => "CONTINUE".to_string(),
        };
    }

    pub fn from(caction_txt: &str) -> ConditionAction {
        return match caction_txt.to_uppercase().as_str() {
            "FAIL" => ConditionAction::FAIL,
            "BREAK" => ConditionAction::BREAK,
            "CONTINUE" => ConditionAction::CONTINUE,
            _ => ConditionAction::FAIL,
        };
    }
}
