use crate::{
    components::routing::submodules::sub_body_data_type::BodyDataType,
    routes::x_utils::definition_store::DefinitionData,
};
use serde_json::{json, value::Value, Map};

use super::convert_bdtype_to_value::bdtype_to_value;

pub fn definition_to_value(value: DefinitionData) -> Value {
    let final_value: Value;

    match value {
        DefinitionData::NULL => {
            final_value = Value::Null;
        }
        DefinitionData::UNDEFINED => {
            final_value = Value::Null;
        }
        DefinitionData::STRING(s) => {
            final_value = Value::String(s);
        }
        DefinitionData::BOOLEAN(b) => {
            final_value = Value::Bool(b);
        }
        DefinitionData::INTEGER(i) => {
            final_value = Value::Number(i.into());
        }
        DefinitionData::FLOAT(f) => {
            final_value = json!(f);
        }
        DefinitionData::DATA(d) => {
            let mut current_value: Map<String, Value> = Map::new();
            current_value.insert(String::from("data_id"), Value::String(d.data_id));
            current_value.insert(String::from("published"), Value::Bool(d.published));

            let mut current_structures = Vec::<Value>::new();
            for structure in d.structures {
                let mut local_value: Map<String, Value> = Map::new();
                local_value.insert(String::from("id"), Value::String(structure.id));
                local_value.insert(
                    String::from("value"),
                    bdtype_to_value(&BodyDataType::from(&structure.rtype), structure.value),
                );
                current_structures.push(Value::Object(local_value));
            }
            current_value.insert(String::from("structures"), Value::Array(current_structures));

            let mut current_custom_structures = Vec::<Value>::new();
            for custom_structure in d.custom_structures {
                let mut local_value: Map<String, Value> = Map::new();
                let mut local_structures = Vec::<Value>::new();

                for structure in custom_structure.structures {
                    let mut local_value: Map<String, Value> = Map::new();
                    local_value.insert(String::from("id"), Value::String(structure.id));
                    local_value.insert(
                        String::from("value"),
                        bdtype_to_value(&BodyDataType::from(&structure.rtype), structure.value),
                    );
                    local_structures.push(Value::Object(local_value));
                }

                local_value.insert(String::from("id"), Value::String(custom_structure.id));
                local_value.insert(String::from("structures"), Value::Array(local_structures));
                current_custom_structures.push(Value::Object(local_value));
            }
            current_value.insert(
                String::from("custom_structures"),
                Value::Array(current_custom_structures),
            );

            final_value = Value::Object(current_value);
        }
        DefinitionData::ARRAY(a) => {
            let mut current_values = Vec::<Value>::new();
            for val in a {
                current_values.push(definition_to_value(val));
            }
            final_value = Value::Array(current_values);
        }
    }

    final_value
}
