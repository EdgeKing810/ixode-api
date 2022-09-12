#![allow(non_camel_case_types)]

use std::fmt;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FunctionList {
    V4,
    GENERATE_TIMESTAMP,
    PAGINATE,
}

impl Default for FunctionList {
    fn default() -> Self {
        FunctionList::V4
    }
}

impl fmt::Display for FunctionList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flist_txt = match self {
            FunctionList::V4 => "V4",
            FunctionList::GENERATE_TIMESTAMP => "GENERATE_TIMESTAMP",
            FunctionList::PAGINATE => "PAGINATE",
        };

        write!(f, "{}", flist_txt)
    }
}

impl FunctionList {
    pub fn to(flist: FunctionList) -> String {
        return match flist.clone() {
            FunctionList::V4 => "V4".to_string(),
            FunctionList::GENERATE_TIMESTAMP => "GENERATE_TIMESTAMP".to_string(),
            FunctionList::PAGINATE => "PAGINATE".to_string(),
        };
    }

    pub fn from(flist_txt: &str) -> FunctionList {
        return match flist_txt.to_uppercase().as_str() {
            "V4" => FunctionList::V4,
            "GENERATE_TIMESTAMP" => FunctionList::GENERATE_TIMESTAMP,
            "PAGINATE" => FunctionList::PAGINATE,
            _ => FunctionList::V4,
        };
    }
}
