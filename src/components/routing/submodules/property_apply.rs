#![allow(non_camel_case_types)]

use std::fmt;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyApply {
    GET_FIRST,
    GET_LAST,
    LENGTH,
    GET_INDEX(u32),
    GET_PROPERTY(String),
}

impl Default for PropertyApply {
    fn default() -> Self {
        PropertyApply::LENGTH
    }
}

impl fmt::Display for PropertyApply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pa_txt = match self {
            PropertyApply::GET_FIRST => "GET_FIRST".to_string(),
            PropertyApply::GET_LAST => "GET_LAST".to_string(),
            PropertyApply::LENGTH => "LENGTH".to_string(),
            PropertyApply::GET_INDEX(x) => x.to_string(),
            PropertyApply::GET_PROPERTY(x) => x.clone(),
        };

        write!(f, "{}", pa_txt)
    }
}

impl PropertyApply {
    pub fn to(pa: PropertyApply) -> String {
        return match pa.clone() {
            PropertyApply::GET_FIRST => "GET_FIRST".to_string(),
            PropertyApply::GET_LAST => "GET_LAST".to_string(),
            PropertyApply::LENGTH => "LENGTH".to_string(),
            PropertyApply::GET_INDEX(x) => x.to_string(),
            PropertyApply::GET_PROPERTY(x) => x.clone(),
        };
    }

    pub fn from(pa_txt: &str) -> PropertyApply {
        return match pa_txt.to_uppercase().as_str() {
            "GET_FIRST" => PropertyApply::GET_FIRST,
            "GET_LAST" => PropertyApply::GET_LAST,
            "LENGTH" => PropertyApply::LENGTH,
            _ => {
                if let Ok(val) = pa_txt.parse::<u32>() {
                    PropertyApply::GET_INDEX(val)
                } else {
                    PropertyApply::GET_PROPERTY(pa_txt.to_string())
                }
            }
        };
    }
}
