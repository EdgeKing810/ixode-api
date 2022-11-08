use crate::components::structures::Type;
use regex::Regex;

use super::{config::get_config_value, mapping::auto_fetch_all_mappings};

pub fn validate_stype(
    data: &str,
    stype: Type,
    is_default: bool,
) -> Result<String, (usize, String)> {
    if is_default && data.len() <= 0 {
        return Ok(String::from("Empty String. Validation Passed!"));
    }

    if stype == Type::EMAIL {
        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        if !email_regex.is_match(data) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid email address", data),
            ));
        }
    } else if stype == Type::INTEGER {
        if let Err(_) = data.parse::<isize>() {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid integer", data),
            ));
        }
    } else if stype == Type::FLOAT {
        if let Err(_) = data.parse::<f64>() {
            return Err((400, format!("Error: Value '{}' is an invalid float", data)));
        }
    } else if stype == Type::DATE {
        let date_regex = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap();

        if !date_regex.is_match(data) {
            return Err((400, format!("Error: Value '{}' is an invalid date", data)));
        }
    } else if stype == Type::DATETIME {
        let datetime_regex = Regex::new(
            r"^[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2} (\+|\-)[0-9]{2}:[0-9]{2}$",
        )
        .unwrap();

        if !datetime_regex.is_match(data) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid datetime", data),
            ));
        }
    } else if stype == Type::MEDIA {
        let mappings = auto_fetch_all_mappings();
        let api_url = get_config_value(&mappings, "API_URL", "none").to_lowercase();

        if api_url != String::from("none") && !data.trim().to_lowercase().starts_with(&api_url) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid media url", data),
            ));
        }
    } else if stype == Type::UID {
        let uid_regex = Regex::new(r"^(?:[a-zA-Z0-9]{1,20}-){3}[a-zA-Z0-9]{1,20}$").unwrap();

        if !uid_regex.is_match(data) {
            return Err((400, format!("Error: Value '{}' is an invalid uid", data)));
        }
    } else if stype == Type::JSON {
        if let Err(_) = serde_json::from_str::<serde_json::Value>(&data) {
            return Err((
                400,
                format!("Error: Value '{}' is an invalid json object", data),
            ));
        }
    }

    Ok(String::from("Validation OK!"))
}
