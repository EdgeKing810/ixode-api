use super::{
    constraint_property::ConstraintProperty,
    io::{fetch_file, save_file},
};
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub component_name: String,
    pub properties: Vec<ConstraintProperty>,
}

impl Constraint {
    pub fn get(
        all_constraints: &Vec<Constraint>,
        component_name: &str,
    ) -> Result<Constraint, (usize, String)> {
        for constraint in all_constraints.iter() {
            if constraint.component_name.to_lowercase() == component_name.to_lowercase() {
                return Ok(constraint.clone());
            }
        }

        Err((404, String::from("Error: Constraint not found")))
    }

    pub fn exist(all_constraints: &Vec<Constraint>, component_name: &str) -> bool {
        for constraint in all_constraints.iter() {
            if constraint.component_name.to_lowercase() == component_name.to_lowercase() {
                return true;
            }
        }

        false
    }

    pub fn create(
        all_constraints: &mut Vec<Constraint>,
        component_name: &str,
    ) -> Result<(), (usize, String)> {
        let tmp_name = String::from("test;");
        let mut new_name = String::from(component_name);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_constraint = Constraint {
            component_name: tmp_name.clone(),
            properties: vec![],
        };
        all_constraints.push(new_constraint);

        let name_update = Self::update_name(all_constraints, &tmp_name, component_name);
        if let Err(e) = name_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_name = tmp_name.clone();
        }

        if has_error {
            let delete_constraint = Self::delete(all_constraints, &new_name);
            if let Err(e) = delete_constraint {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn update_name(
        all_constraints: &mut Vec<Constraint>,
        old_component_name: &str,
        component_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_constraint: Option<Constraint> = None;

        if !String::from(component_name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ' ')
        {
            return Err((
                400,
                String::from("Error: component_name contains an invalid character"),
            ));
        }

        if String::from(component_name.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: component_name does not contain enough characters"),
            ));
        } else if String::from(component_name.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: component_name contains too many characters"),
            ));
        }

        for constraint in all_constraints.iter_mut() {
            if constraint.component_name == *old_component_name {
                found_constraint = Some(constraint.clone());
                constraint.component_name = component_name.trim().to_string();
                break;
            }
        }

        if let None = found_constraint {
            return Err((404, String::from("Error: Constraint not found")));
        }

        Ok(())
    }

    pub fn add_property(
        all_constraints: &mut Vec<Constraint>,
        component_name: &str,
        property: ConstraintProperty,
    ) -> Result<(), (usize, String)> {
        let mut found_constraint: Option<Constraint> = None;

        for constraint in all_constraints.iter_mut() {
            if constraint.component_name == *component_name {
                found_constraint = Some(constraint.clone());

                let mut current_properties = constraint.properties.clone();

                match ConstraintProperty::create(
                    &mut current_properties,
                    &property.property_name,
                    property.is_alphabetic,
                    property.is_numeric,
                    property.min,
                    property.max,
                    property.not_allowed,
                    property.additional_allowed,
                ) {
                    Err(e) => return Err(e),
                    _ => {}
                }

                constraint.properties = current_properties;

                break;
            }
        }

        if let None = found_constraint {
            return Err((404, String::from("Error: Constraint not found")));
        }

        Ok(())
    }

    pub fn update_property(
        all_constraints: &mut Vec<Constraint>,
        component_name: &str,
        property_name: &str,
        property: ConstraintProperty,
    ) -> Result<(), (usize, String)> {
        let mut found_constraint: Option<Constraint> = None;

        for constraint in all_constraints.iter_mut() {
            if constraint.component_name == *component_name {
                found_constraint = Some(constraint.clone());

                let mut current_properties = constraint.properties.clone();
                let mut updated_properties = Vec::<ConstraintProperty>::new();

                for current_property in current_properties.iter_mut() {
                    if current_property.property_name != *property_name {
                        updated_properties.push(current_property.clone());
                    } else {
                        match ConstraintProperty::create(
                            &mut updated_properties,
                            &property.property_name,
                            property.is_alphabetic,
                            property.is_numeric,
                            property.min,
                            property.max,
                            property.not_allowed.clone(),
                            property.additional_allowed.clone(),
                        ) {
                            Err(e) => return Err(e),
                            _ => {}
                        }
                    }
                }

                constraint.properties = updated_properties;

                break;
            }
        }

        if let None = found_constraint {
            return Err((404, String::from("Error: Constraint not found")));
        }

        Ok(())
    }

    pub fn set_properties(
        all_constraints: &mut Vec<Constraint>,
        component_name: &str,
        properties: Vec<ConstraintProperty>,
    ) -> Result<(), (usize, String)> {
        let mut found_constraint: Option<Constraint> = None;

        for constraint in all_constraints.iter_mut() {
            if constraint.component_name == *component_name {
                found_constraint = Some(constraint.clone());
                constraint.properties = properties;

                break;
            }
        }

        if let None = found_constraint {
            return Err((404, String::from("Error: Constraint not found")));
        }

        Ok(())
    }

    pub fn remove_property(
        all_constraints: &mut Vec<Constraint>,
        component_name: &str,
        property_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_constraint: Option<Constraint> = None;

        for constraint in all_constraints.iter_mut() {
            if constraint.component_name == *component_name {
                found_constraint = Some(constraint.clone());

                let mut current_properties = constraint.properties.clone();
                match ConstraintProperty::delete(&mut current_properties, property_name) {
                    Err(e) => return Err(e),
                    _ => {}
                }

                constraint.properties = current_properties;
                break;
            }
        }

        if let None = found_constraint {
            return Err((404, String::from("Error: Constraint not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_constraints: &mut Vec<Constraint>,
        component_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_constraint: Option<Constraint> = None;

        for constraint in all_constraints.iter_mut() {
            if constraint.component_name == component_name.to_string() {
                found_constraint = Some(constraint.clone());
                break;
            }
        }

        if let None = found_constraint {
            return Err((404, String::from("Error: Constraint not found")));
        }

        let updated_constraints: Vec<Constraint> = all_constraints
            .iter_mut()
            .filter(|constraint| constraint.component_name != *component_name)
            .map(|constraint| Constraint {
                component_name: constraint.component_name.clone(),
                properties: constraint.properties.clone(),
            })
            .collect::<Vec<Constraint>>();

        *all_constraints = updated_constraints;

        Ok(())
    }

    pub fn to_string_debug(constraint: Constraint) -> String {
        let mut final_string = format!("{}:\n", constraint.component_name);
        for property in constraint.properties {
            let mut final_additional_allowed = String::new();
            for (i, c) in property.additional_allowed.iter().enumerate() {
                final_additional_allowed.push_str(&format!(
                    "+ {}{}",
                    if c == &' ' {
                        "<space>".to_string()
                    } else {
                        c.to_string()
                    },
                    if i < property.additional_allowed.len() - 1 {
                        " "
                    } else {
                        ""
                    }
                ));
            }

            let mut final_not_allowed = String::new();
            for (i, c) in property.not_allowed.iter().enumerate() {
                final_not_allowed.push_str(&format!(
                    "{}{}",
                    if c == &' ' {
                        "<space>".to_string()
                    } else {
                        c.to_string()
                    },
                    if i < property.not_allowed.len() - 1 {
                        " "
                    } else {
                        ""
                    }
                ));
            }

            final_string = format!(
                "{}  {}: {}{}{} ({}, {})\n",
                final_string,
                property.property_name,
                if property.is_alphabetic && property.is_numeric {
                    "alphanumeric "
                } else if property.is_alphabetic {
                    "alphabetic "
                } else if property.is_numeric {
                    "numeric "
                } else {
                    ""
                },
                final_additional_allowed,
                if final_not_allowed.len() <= 0 {
                    String::new()
                } else {
                    format!("NOT[{}]", final_not_allowed)
                },
                property.min,
                property.max
            );
        }
        final_string
    }

    pub fn to_string(constraint: Constraint) -> String {
        let stringified_properties = ConstraintProperty::stringify(&constraint.properties);

        format!("{};{}", constraint.component_name, stringified_properties)
    }

    pub fn from_string(
        all_constraints: &mut Vec<Constraint>,
        constraint_str: &str,
    ) -> Result<(), (usize, String)> {
        let current_constraint = constraint_str.split(";").collect::<Vec<&str>>();

        let component_name = current_constraint[0];

        let mut all_properties = Vec::<ConstraintProperty>::new();
        let properties_str = current_constraint[1..].join(";");
        for property_str in properties_str.split("§").collect::<Vec<&str>>() {
            if let Err(e) = ConstraintProperty::from_string(&mut all_properties, property_str) {
                return Err(e);
            }
        }

        if let Err(e) = Constraint::create(all_constraints, component_name) {
            return Err(e);
        }

        if let Err(e) = Constraint::set_properties(all_constraints, component_name, all_properties)
        {
            return Err(e);
        }

        Ok(())
    }
}

pub fn stringify_constraints(all_constraints: &Vec<Constraint>) -> String {
    let mut stringified_constraints = String::new();

    for constraint in all_constraints {
        stringified_constraints = format!(
            "{}{}{}",
            stringified_constraints,
            if stringified_constraints.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Constraint::to_string(constraint.clone())
        );
    }

    stringified_constraints
}

pub fn stringify_constraints_debug(all_constraints: &Vec<Constraint>) -> String {
    let mut stringified_constraints = String::new();

    for constraint in all_constraints {
        stringified_constraints = format!(
            "{}{}{}",
            stringified_constraints,
            if stringified_constraints.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Constraint::to_string_debug(constraint.clone())
        );
    }

    stringified_constraints
}

pub fn unwrap_constraints(all_constraints_raw: String) -> Result<Vec<Constraint>, (usize, String)> {
    let individual_constraints = all_constraints_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_constraints = Vec::<Constraint>::new();

    for constraint in individual_constraints {
        if let Err(e) = Constraint::from_string(&mut final_constraints, constraint) {
            return Err(e);
        }
    }

    Ok(final_constraints)
}

pub fn fetch_all_constraints(
    path: String,
    encryption_key: &String,
) -> Result<Vec<Constraint>, (usize, String)> {
    let all_constraints_raw = fetch_file(path.clone(), encryption_key);
    unwrap_constraints(all_constraints_raw)
}

pub fn save_all_constraints(
    all_constraints: &Vec<Constraint>,
    path: String,
    encryption_key: &String,
) {
    let stringified_constraints = stringify_constraints(all_constraints);
    save_file(path, stringified_constraints, encryption_key);
    println!("Constraints saved!");
}
