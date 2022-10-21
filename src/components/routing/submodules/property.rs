use super::{sub_property_apply::PropertyApply, sub_ref_data::RefData};
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Property {
    pub data: RefData,
    pub apply: PropertyApply,
    pub additional: String,
}

impl Default for Property {
    fn default() -> Self {
        Property {
            data: RefData::default(),
            apply: PropertyApply::default(),
            additional: "".to_string(),
        }
    }
}

impl Property {
    pub fn create(
        data: RefData,
        apply: &str,
        additional: &str,
    ) -> Result<Property, (usize, String)> {
        if apply.trim().len() > 100 {
            return Err((
                400,
                String::from("Error: apply contains too many characters"),
            ));
        }

        if !apply
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
        {
            return Err((
                400,
                String::from("Error: data contains an invalid character"),
            ));
        }

        if !additional
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
        {
            return Err((
                400,
                String::from("Error: additional contains an invalid character"),
            ));
        }

        if additional.trim().len() > 100 {
            return Err((
                400,
                String::from("Error: additional contains too many characters"),
            ));
        }

        Ok(Property {
            data: data,
            apply: PropertyApply::from(apply),
            additional: additional.trim().to_string(),
        })
    }

    pub fn from_string(property_str: &str) -> Result<Property, (usize, String)> {
        let mut current_property = property_str.split("(").collect::<Vec<&str>>();
        if current_property.len() <= 1 {
            return Err((500, String::from("Invalid property (at declaration start)")));
        }

        current_property = current_property[1].split(")").collect::<Vec<&str>>();
        if current_property.len() <= 1 {
            return Err((500, String::from("Invalid property (at declaration end)")));
        }

        current_property = current_property[0].split("|").collect::<Vec<&str>>();
        if current_property.len() < 2 {
            return Err((500, String::from("Invalid property (in format)")));
        }

        let mut apply_str = current_property[1].split("apply=").collect::<Vec<&str>>();
        if apply_str.len() <= 1 {
            return Err((500, String::from("Invalid property (in 'apply' format)")));
        }

        apply_str = apply_str[1].split(">").collect::<Vec<&str>>();
        if apply_str.len() <= 1 {
            return Err((500, String::from("Invalid property (in 'apply' format)")));
        }

        let right = match RefData::from_string(current_property[0]) {
            Ok(right) => right,
            Err(err) => {
                return Err((
                    500,
                    format!("Invalid property (in 'data' format) -> {}", err.1),
                ))
            }
        };

        match Property::create(right, apply_str[0], apply_str[1]) {
            Ok(property) => Ok(property),
            Err(err) => Err((
                500,
                format!("Invalid property (while processing) -> {}", err.1),
            )),
        }
    }

    pub fn to_string(property: Property) -> String {
        format!(
            "({}|apply={}>{})",
            RefData::to_string(property.data.clone()),
            PropertyApply::to(property.apply.clone()),
            property.additional
        )
    }
}
