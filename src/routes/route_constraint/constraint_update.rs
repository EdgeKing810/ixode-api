use rocket::patch;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use crate::components::constraint::Constraint;
use crate::components::constraint_property::ConstraintProperty;
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::constraint::{auto_fetch_all_constraints, auto_save_all_constraints};
use crate::utils::event::auto_create_event;
use crate::utils::mapping::auto_fetch_all_mappings;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(crate = "rocket::serde")]
pub enum UpdateType {
    MIN,
    MAX,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateConstraintInput {
    uid: String,
    component_name: String,
    property_name: String,
    change: UpdateType,
    data: usize,
}

#[patch("/update", format = "json", data = "<data>")]
pub async fn main(data: Json<UpdateConstraintInput>, token: Token) -> Value {
    let uid = &data.uid;
    let component_name = &data.component_name;
    let property_name = &data.property_name;
    let change = &data.change;
    let data = &data.data;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();

    let all_constraints = match auto_fetch_all_constraints(&mappings) {
        Ok(c) => c,
        Err(_) => return json!({"status": 500, "message": "Error: Failed fetching constraints"}),
    };

    let constraint = match Constraint::get(&all_constraints, &component_name) {
        Ok(c) => c,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Constraint with this component_name found"})
        }
    };

    let mut all_properties = constraint.properties.clone();
    let property = match ConstraintProperty::get(&all_properties, &property_name) {
        Ok(p) => p,
        Err(_) => {
            return json!({"status": 404, "message": "Error: No Constraint Property with this property_name found"})
        }
    };

    match match change.clone() {
        UpdateType::MIN => {
            ConstraintProperty::update_min(&mut all_properties, &property_name, *data)
        }
        UpdateType::MAX => {
            ConstraintProperty::update_max(&mut all_properties, &property_name, *data)
        }
    } {
        Err(e) => return json!({"status": e.0, "message": e.1}),
        _ => {}
    }

    if change.clone() == &UpdateType::MIN {
        if let Err(e) = auto_create_event(
            &mappings,
            "constraint_update_min",
            format!(
                "The min value of the constraint <{}.{}> was updated from <{}> to <{}> by usr[{}]",
                component_name, property_name, property.min, data, uid
            ),
            String::from("/constraints"),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    } else if change.clone() == &UpdateType::MAX {
        if let Err(e) = auto_create_event(
            &mappings,
            "constraint_update_max",
            format!(
                "The max value of the constraint <{}.{}> was updated from <{}> to <{}> by usr[{}]",
                component_name, property_name, property.max, data, uid
            ),
            String::from("/constraints"),
        ) {
            return json!({"status": e.0, "message": e.1});
        }
    }

    match auto_save_all_constraints(&mappings, &all_constraints) {
        Ok(_) => return json!({"status": 200, "message": "Constraint successfully updated!"}),
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
