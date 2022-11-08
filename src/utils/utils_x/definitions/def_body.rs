use serde_json::Value;

use crate::{
    components::routing::{
        core::core_body_data::BodyData, submodules::sub_body_data_type::BodyDataType,
    },
    utils::x::definition_store::DefinitionData,
};

pub fn define_body(
    actual_body: &Value,
    current_body: BodyData,
) -> Result<DefinitionData, (usize, String)> {
    if !actual_body.is_object() {
        return Err((
            400,
            String::from("Error: Body data is not in object format"),
        ));
    }

    let payload = actual_body[current_body.id.clone()].clone();
    if payload.is_null() {
        return Err((
            400,
            format!("Error: Body data field '{}' is absent", current_body.id),
        ));
    }

    let data = match current_body.bdtype {
        BodyDataType::STRING => {
            if !payload.is_string() {
                return Err((
                    400,
                    format!(
                        "Error: Body data field '{}' is not in string format",
                        current_body.id
                    ),
                ));
            }
            DefinitionData::STRING(payload.as_str().unwrap().to_string())
        }
        BodyDataType::INTEGER => {
            if !payload.is_i64() {
                return Err((
                    400,
                    format!(
                        "Error: Body data field '{}' is not in integer format",
                        current_body.id
                    ),
                ));
            }
            DefinitionData::INTEGER(payload.as_i64().unwrap() as isize)
        }
        BodyDataType::FLOAT => {
            if !payload.is_f64() {
                return Err((
                    400,
                    format!(
                        "Error: Body data field '{}' is not in float format",
                        current_body.id
                    ),
                ));
            }
            DefinitionData::FLOAT(payload.as_f64().unwrap())
        }
        BodyDataType::BOOLEAN => {
            if !payload.is_boolean() {
                return Err((
                    400,
                    format!(
                        "Error: Body data field '{}' is not in boolean format",
                        current_body.id
                    ),
                ));
            }
            DefinitionData::BOOLEAN(payload.as_bool().unwrap())
        }
        BodyDataType::ARRAY => {
            if !payload.is_array() {
                return Err((
                    400,
                    format!(
                        "Error: Body data field '{}' is not in boolean format",
                        current_body.id
                    ),
                ));
            }

            let mut all_definitions = Vec::<DefinitionData>::new();
            for v in payload.as_array().unwrap() {
                if v.is_string() {
                    all_definitions.push(DefinitionData::STRING(v.as_str().unwrap().to_string()));
                } else if v.is_i64() {
                    all_definitions.push(DefinitionData::INTEGER(v.as_i64().unwrap() as isize));
                } else if v.is_f64() {
                    all_definitions.push(DefinitionData::FLOAT(v.as_f64().unwrap()));
                } else if v.is_boolean() {
                    all_definitions.push(DefinitionData::BOOLEAN(v.as_bool().unwrap()));
                }
            }

            DefinitionData::ARRAY(all_definitions)
        }
        _ => {
            return Err((
                400,
                format!(
                    "Error: Body data field '{}' is not in a valid format",
                    current_body.id
                ),
            ));
        }
    };

    Ok(data)
}
