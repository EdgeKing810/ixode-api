use crate::components::routing::submodules::sub_body_data_type::BodyDataType;
use serde_json::{json, value::Value};

pub fn bdtype_to_value(bdtype: &BodyDataType, value: String) -> Value {
    let mut final_value = Value::Null;

    match bdtype {
        BodyDataType::STRING => {
            final_value = Value::String(value);
        }
        BodyDataType::BOOLEAN => match value.as_str() {
            "true" => {
                final_value = Value::Bool(true);
            }
            "false" => {
                final_value = Value::Bool(false);
            }
            _ => {}
        },
        BodyDataType::INTEGER => match value.parse::<i64>() {
            Ok(i) => {
                final_value = Value::Number(i.into());
            }
            _ => {}
        },
        BodyDataType::FLOAT => match value.parse::<f64>() {
            Ok(f) => {
                final_value = json!(f);
            }
            _ => {}
        },
        BodyDataType::ARRAY => {
            let mut current_values = Vec::<Value>::new();
            for s in value.split(",") {
                current_values.push(Value::String(String::from(s.trim())));
            }
            final_value = Value::Array(current_values);
        }
        BodyDataType::OTHER => {
            final_value = Value::String(value);
        }
    }

    final_value
}
