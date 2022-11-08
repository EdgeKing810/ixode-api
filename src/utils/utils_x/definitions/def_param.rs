use crate::{
    components::routing::{
        core::core_param_data::ParamData, submodules::sub_body_data_type::BodyDataType,
    },
    routes::x::x::LocalParamData,
    utils::x::definition_store::DefinitionData,
};

pub fn define_param(
    params: &ParamData,
    all_params: &Vec<LocalParamData>,
    index: usize,
) -> Result<DefinitionData, (usize, String)> {
    if index >= params.pairs.len() {
        return Ok(DefinitionData::NULL);
    }
    let pair = params.pairs[index].clone();

    let mut current_value = String::new();
    for local_param_data in all_params {
        if local_param_data.key == pair.id {
            current_value = local_param_data.value.clone();
            break;
        }
    }

    if current_value.trim().len() <= 0 {
        return Ok(DefinitionData::NULL);
    }

    let data = match pair.bdtype {
        BodyDataType::STRING => DefinitionData::STRING(current_value),
        BodyDataType::INTEGER => DefinitionData::INTEGER(match current_value.parse::<isize>() {
            Ok(value) => value,
            Err(_) => {
                return Err((
                    400,
                    format!("Error: Invalid integer value for parameter '{}'", pair.id),
                ))
            }
        }),
        BodyDataType::FLOAT => DefinitionData::FLOAT(match current_value.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                return Err((
                    400,
                    format!("Invalid float value for parameter '{}'", pair.id),
                ))
            }
        }),
        BodyDataType::BOOLEAN => DefinitionData::BOOLEAN(match current_value.parse::<bool>() {
            Ok(value) => value,
            Err(_) => {
                return Err((
                    400,
                    format!("Invalid boolean value for parameter '{}'", pair.id),
                ))
            }
        }),
        BodyDataType::ARRAY => {
            let broken_current_value = current_value.split(",");
            let mut all_definitions = Vec::<DefinitionData>::new();
            for bc_val in broken_current_value {
                if bc_val.trim().parse::<isize>().is_ok() {
                    all_definitions.push(DefinitionData::INTEGER(
                        bc_val.trim().parse::<isize>().unwrap(),
                    ));
                } else if bc_val.trim().parse::<f64>().is_ok() {
                    all_definitions
                        .push(DefinitionData::FLOAT(bc_val.trim().parse::<f64>().unwrap()));
                } else if bc_val.trim().parse::<bool>().is_ok() {
                    all_definitions.push(DefinitionData::BOOLEAN(
                        bc_val.trim().parse::<bool>().unwrap(),
                    ));
                } else {
                    all_definitions.push(DefinitionData::STRING(bc_val.trim().to_string()));
                }
            }
            DefinitionData::ARRAY(all_definitions)
        }
        _ => {
            return Err((
                400,
                format!("Invalid data type for parameter '{}'", pair.id),
            ))
        }
    };

    Ok(data)
}
