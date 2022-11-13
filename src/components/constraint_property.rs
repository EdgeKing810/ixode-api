use super::constraint::Constraint;
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintProperty {
    pub property_name: String,
    pub is_alphabetic: bool,
    pub is_numeric: bool,
    pub min: usize,
    pub max: usize,
    pub not_allowed: Vec<char>,
    pub additional_allowed: Vec<char>,
}

impl ConstraintProperty {
    pub fn create(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        is_alphabetic: bool,
        is_numeric: bool,
        min: usize,
        max: usize,
        not_allowed: Vec<char>,
        additional_allowed: Vec<char>,
    ) -> Result<(), (usize, String)> {
        let tmp_name = String::from("test;");
        let mut new_name = String::from(property_name);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        if ConstraintProperty::exist(all_properties, property_name) {
            return Err((
                403,
                format!(
                    "Error: A constraint property with that property_name already exists ({})",
                    property_name
                ),
            ));
        }

        let new_property = ConstraintProperty {
            property_name: tmp_name.clone(),
            is_alphabetic: false,
            is_numeric: false,
            min: 0,
            max: 0,
            not_allowed: vec![],
            additional_allowed: vec![],
        };
        all_properties.push(new_property);

        let property_name_update =
            Self::update_property_name(all_properties, &tmp_name, property_name);
        if let Err(e) = property_name_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_name = tmp_name.clone();
        }

        if !has_error {
            let is_alphabetic_update =
                Self::update_is_alphabetic(all_properties, &new_name, is_alphabetic);
            if let Err(e) = is_alphabetic_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let is_numeric_update = Self::update_is_numeric(all_properties, &new_name, is_numeric);
            if let Err(e) = is_numeric_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let min_update = Self::update_min(all_properties, &new_name, min);
            if let Err(e) = min_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let max_update = Self::update_max(all_properties, &new_name, max);
            if let Err(e) = max_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let not_allowed_update = Self::set_not_allowed(all_properties, &new_name, not_allowed);
            if let Err(e) = not_allowed_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let additional_allowed_update =
                Self::set_additional_allowed(all_properties, &new_name, additional_allowed);
            if let Err(e) = additional_allowed_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_property = Self::delete(all_properties, &new_name);
            if let Err(e) = delete_property {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn get(
        all_properties: &Vec<ConstraintProperty>,
        property_name: &str,
    ) -> Result<ConstraintProperty, (usize, String)> {
        for property in all_properties.iter() {
            if property.property_name.to_lowercase() == property_name.to_lowercase() {
                return Ok(property.clone());
            }
        }

        Err((404, String::from("Error: Constraint Property not found")))
    }

    pub fn exist(all_properties: &Vec<ConstraintProperty>, property_name: &str) -> bool {
        for property in all_properties.iter() {
            if property.property_name.to_lowercase() == property_name.to_lowercase() {
                return true;
            }
        }

        false
    }

    pub fn update_property_name(
        all_properties: &mut Vec<ConstraintProperty>,
        old_property_name: &str,
        property_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        if !String::from(property_name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ' ')
        {
            return Err((
                400,
                String::from("Error: property_name contains an invalid character"),
            ));
        }

        if String::from(property_name.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: property_name does not contain enough characters"),
            ));
        } else if String::from(property_name.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: property_name contains too many characters"),
            ));
        }

        for property in all_properties.iter_mut() {
            if property.property_name == *old_property_name {
                found_property = Some(property.clone());
                property.property_name = property_name.trim().to_string();
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn update_is_alphabetic(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        is_alphabetic: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());
                property.is_alphabetic = is_alphabetic;
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn update_is_numeric(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        is_numeric: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());
                property.is_numeric = is_numeric;
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn update_min(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        min: usize,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());
                property.min = min;
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn update_max(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        max: usize,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());
                property.max = max;
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn add_not_allowed(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        character: char,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        if character == '\n' || character == '\r' || character == '§' {
            return Err((400, String::from("Error: Invalid value for character")));
        }

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());

                if property.not_allowed.contains(&character) {
                    return Err((400, String::from("Error: Character is already not allowed")));
                }

                property.not_allowed.push(character);

                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn set_not_allowed(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        not_allowed: Vec<char>,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());
                property.not_allowed = not_allowed;
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn remove_not_allowed(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        character: char,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());

                property.not_allowed = property
                    .not_allowed
                    .iter()
                    .filter(|&c| *c != character)
                    .cloned()
                    .collect();
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn add_additional_allowed(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        character: char,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        if character == '\n' || character == '\r' || character == '§' {
            return Err((400, String::from("Error: Invalid value for character")));
        }

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());

                if property.additional_allowed.contains(&character) {
                    return Err((400, String::from("Error: Character is already allowed")));
                }

                property.additional_allowed.push(character);

                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn set_additional_allowed(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        additional_allowed: Vec<char>,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());
                property.additional_allowed = additional_allowed;
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn remove_additional_allowed(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
        character: char,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == *property_name {
                found_property = Some(property.clone());

                property.additional_allowed = property
                    .additional_allowed
                    .iter()
                    .filter(|&c| *c != character)
                    .cloned()
                    .collect();
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        Ok(())
    }

    pub fn validate(
        all_constraints: &Vec<Constraint>,
        component_name: &str,
        property_name: &str,
        value: &str,
    ) -> Result<String, (usize, String)> {
        let constraint = match Constraint::get(all_constraints, component_name) {
            Ok(constraint) => constraint,
            Err(err) => return Err(err),
        };

        let property = match ConstraintProperty::get(&constraint.properties, property_name) {
            Ok(property) => property,
            Err(err) => return Err(err),
        };

        let mut final_value = value.trim().to_string();

        if final_value.len() < property.min {
            return Err((
                400,
                format!(
                    "Error: {} does not contain enough characters",
                    property_name
                ),
            ));
        }

        if final_value.len() > property.max {
            return Err((
                400,
                format!("Error: {} contains too many characters", property_name),
            ));
        }

        for c in property.not_allowed {
            final_value = final_value.replace(c, "_");
        }
        final_value = final_value.replace('\n', "_newline_");

        let mut final_value_check = final_value.clone();
        for a in property.additional_allowed {
            final_value_check = final_value_check.replace(a, "");
        }

        let is_alphanumeric = property.is_alphabetic && property.is_numeric;

        if is_alphanumeric {
            if !final_value_check.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err((
                    400,
                    format!("Error: {} contains an invalid character", property_name),
                ));
            }
        } else if property.is_alphabetic {
            if !final_value_check.chars().all(|c| c.is_ascii_alphabetic()) {
                return Err((
                    400,
                    format!("Error: {} contains an invalid character", property_name),
                ));
            }
        } else if property.is_numeric {
            if !final_value_check.chars().all(|c| c.is_ascii_digit()) {
                return Err((
                    400,
                    format!("Error: {} contains an invalid character", property_name),
                ));
            }
        }

        Ok(final_value.trim().to_string())
    }

    pub fn delete(
        all_properties: &mut Vec<ConstraintProperty>,
        property_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_property: Option<ConstraintProperty> = None;

        for property in all_properties.iter_mut() {
            if property.property_name == property_name.to_string() {
                found_property = Some(property.clone());
                break;
            }
        }

        if let None = found_property {
            return Err((404, String::from("Error: Constraint Property not found")));
        }

        let updated_properties: Vec<ConstraintProperty> = all_properties
            .iter_mut()
            .filter(|property| property.property_name != *property_name)
            .map(|property| ConstraintProperty {
                property_name: property.property_name.clone(),
                is_alphabetic: property.is_alphabetic,
                is_numeric: property.is_numeric,
                min: property.min,
                max: property.max,
                not_allowed: property.not_allowed.clone(),
                additional_allowed: property.additional_allowed.clone(),
            })
            .collect::<Vec<ConstraintProperty>>();

        *all_properties = updated_properties;

        Ok(())
    }

    pub fn stringify(all_properties: &Vec<ConstraintProperty>) -> String {
        let mut stringified_properties = String::new();

        for property in all_properties {
            stringified_properties = format!(
                "{}{}{}",
                stringified_properties,
                if stringified_properties.chars().count() > 1 {
                    "§"
                } else {
                    ""
                },
                ConstraintProperty::to_string(property.clone())
            );
        }

        stringified_properties
    }

    pub fn to_string(property: ConstraintProperty) -> String {
        format!(
            "{};{};{};{};{};not_allowed={};allowed={}",
            property.property_name,
            if property.is_alphabetic {
                "true"
            } else {
                "false"
            },
            if property.is_numeric { "true" } else { "false" },
            property.min,
            property.max,
            property.not_allowed.iter().collect::<String>(),
            property.additional_allowed.iter().collect::<String>()
        )
    }

    pub fn from_string(
        mut all_properties: &mut Vec<ConstraintProperty>,
        property_str: &str,
    ) -> Result<(), (usize, String)> {
        let current_property = property_str.split(";").collect::<Vec<&str>>();

        let property_name = current_property[0];
        let is_alphabetic = current_property[1] == "true";
        let is_numeric = current_property[2] == "true";
        let min = current_property[3].parse::<usize>().unwrap();
        let max = current_property[4].parse::<usize>().unwrap();

        let not_allowed_tmp = property_str.split(";not_allowed=").collect::<Vec<&str>>();
        let not_allowed = not_allowed_tmp[1].split(";allowed=").collect::<Vec<&str>>()[0]
            .chars()
            .collect::<Vec<char>>();

        let additional_allowed = not_allowed_tmp[1].split(";allowed=").collect::<Vec<&str>>()[1]
            .chars()
            .collect::<Vec<char>>();

        ConstraintProperty::create(
            &mut all_properties,
            property_name,
            is_alphabetic,
            is_numeric,
            min,
            max,
            not_allowed,
            additional_allowed,
        )
    }
}
