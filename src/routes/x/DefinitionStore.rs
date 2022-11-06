use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;
use crate::components::routing::submodules::sub_condition::Condition;
use crate::components::routing::submodules::sub_condition_type::ConditionType;
use crate::components::routing::submodules::sub_function_list::FunctionList;
use crate::components::routing::submodules::sub_property_apply::PropertyApply;
use crate::middlewares::paginate::paginate;

use chrono::prelude::*;
use serde_json::Value;
use uuid::Uuid;

use crate::components::routing::mod_route::RouteComponent;
use crate::components::routing::submodules::sub_body_data_type::BodyDataType;
use crate::components::routing::submodules::sub_next_condition_type::NextConditionType;
use crate::components::routing::submodules::sub_operation::Operation;
use crate::components::routing::submodules::sub_operation_type::OperationType;
use crate::components::routing::submodules::sub_ref_data::RefData;
use crate::data_converter::{
    convert_from_raw, convert_to_raw, CustomStructurePair, RawPair, StructurePair,
};
use crate::routes::x::LocalParamData;
use crate::utils::{
    auto_fetch_all_collections, auto_fetch_all_data, auto_fetch_all_mappings, auto_save_all_data,
};

use super::global_block_order::GlobalBlockOrder;
use super::resolver::{resolve_conditions, resolve_operations, resolve_raw_data, resolve_ref_data};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefinitionStore {
    pub block_name: String,
    pub ref_name: String,
    pub index: usize,
    pub data: DefinitionData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DefinitionData {
    NULL,
    UNDEFINED,
    BOOLEAN(bool),
    STRING(String),
    INTEGER(isize),
    FLOAT(f64),
    ARRAY(Vec<DefinitionData>),
    DATA(RawPair),
}

impl DefinitionStore {
    pub fn add_definition(
        current_route: &RouteComponent,
        all_definitions: &mut Vec<DefinitionStore>,
        global_blocks: &Vec<GlobalBlockOrder>,
        project_id: &str,
        block_name: &str,
        index: usize,
        current_index: usize,
        actual_body: &Value,
        all_params: &Vec<LocalParamData>,
    ) -> Result<(), (usize, String)> {
        let mut actual_definition = DefinitionStore {
            block_name: block_name.to_string(),
            ref_name: String::from(""),
            index: index,
            data: DefinitionData::NULL,
        };

        if block_name == "BODY" {
            let current_body = current_route.body[index].clone();
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
                            all_definitions
                                .push(DefinitionData::STRING(v.as_str().unwrap().to_string()));
                        } else if v.is_i64() {
                            all_definitions
                                .push(DefinitionData::INTEGER(v.as_i64().unwrap() as isize));
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

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: current_body.id,
                index: index,
                data: data,
            };
        } else if block_name == "PARAM" {
            if let Some(params) = &current_route.params {
                if index < params.pairs.len() {
                    let pair = params.pairs[index].clone();

                    let mut current_value = String::new();
                    for local_param_data in all_params {
                        if local_param_data.key == pair.id {
                            current_value = local_param_data.value.clone();
                            break;
                        }
                    }

                    if current_value.trim().len() > 0 {
                        let data = match pair.bdtype {
                            BodyDataType::STRING => DefinitionData::STRING(current_value),
                            BodyDataType::INTEGER => {
                                DefinitionData::INTEGER(match current_value.parse::<isize>() {
                                    Ok(value) => value,
                                    Err(_) => {
                                        return Err((
                                            400,
                                            format!(
                                                "Error: Invalid integer value for parameter '{}'",
                                                pair.id
                                            ),
                                        ))
                                    }
                                })
                            }
                            BodyDataType::FLOAT => {
                                DefinitionData::FLOAT(match current_value.parse::<f64>() {
                                    Ok(value) => value,
                                    Err(_) => {
                                        return Err((
                                            400,
                                            format!(
                                                "Invalid float value for parameter '{}'",
                                                pair.id
                                            ),
                                        ))
                                    }
                                })
                            }
                            BodyDataType::BOOLEAN => {
                                DefinitionData::BOOLEAN(match current_value.parse::<bool>() {
                                    Ok(value) => value,
                                    Err(_) => {
                                        return Err((
                                            400,
                                            format!(
                                                "Invalid boolean value for parameter '{}'",
                                                pair.id
                                            ),
                                        ))
                                    }
                                })
                            }
                            BodyDataType::ARRAY => {
                                let broken_current_value = current_value.split(",");
                                let mut all_definitions = Vec::<DefinitionData>::new();
                                for bc_val in broken_current_value {
                                    if bc_val.trim().parse::<isize>().is_ok() {
                                        all_definitions.push(DefinitionData::INTEGER(
                                            bc_val.trim().parse::<isize>().unwrap(),
                                        ));
                                    } else if bc_val.trim().parse::<f64>().is_ok() {
                                        all_definitions.push(DefinitionData::FLOAT(
                                            bc_val.trim().parse::<f64>().unwrap(),
                                        ));
                                    } else if bc_val.trim().parse::<bool>().is_ok() {
                                        all_definitions.push(DefinitionData::BOOLEAN(
                                            bc_val.trim().parse::<bool>().unwrap(),
                                        ));
                                    } else {
                                        all_definitions.push(DefinitionData::STRING(
                                            bc_val.trim().to_string(),
                                        ));
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

                        actual_definition = DefinitionStore {
                            block_name: block_name.to_string(),
                            ref_name: pair.id,
                            index: index,
                            data: data,
                        };
                    }
                }
            }
        } else if block_name == "FETCH" {
            let fetch_block = current_route.flow.fetchers[index].clone();

            let mappings = auto_fetch_all_mappings();
            let all_data = match auto_fetch_all_data(&mappings, &project_id, &fetch_block.ref_col) {
                Ok(d) => d,
                _ => {
                    return Err((500, String::from("Error: Failed fetching data")));
                }
            };

            let all_collections = match auto_fetch_all_collections(&mappings) {
                Ok(u) => u,
                _ => {
                    return Err((500, String::from("Error: Failed fetching collections")));
                }
            };

            let collection =
                match Collection::get(&all_collections, project_id, &fetch_block.ref_col) {
                    Ok(p) => p,
                    Err(_) => {
                        return Err((
                            404,
                            String::from("Error: No Collection with this collection_id found"),
                        ));
                    }
                };

            let current_data = Data::get_all(&all_data, project_id, &fetch_block.ref_col);
            let mut all_definitions = Vec::<DefinitionData>::new();

            for data in current_data {
                match convert_to_raw(&data, &collection) {
                    Ok(rp) => {
                        all_definitions.push(DefinitionData::DATA(rp));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                };
            }

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: fetch_block.local_name,
                index: index,
                data: DefinitionData::ARRAY(all_definitions),
            };
        } else if block_name == "ASSIGN" {
            let assign_block = current_route.flow.assignments[index].clone();
            let res_condition = match resolve_conditions(
                &assign_block.conditions,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(c) => c,
                Err(e) => {
                    return Err(e);
                }
            };

            if res_condition {
                match resolve_operations(
                    &assign_block.operations,
                    global_blocks,
                    all_definitions,
                    current_index,
                ) {
                    Ok(d) => {
                        actual_definition = DefinitionStore {
                            block_name: block_name.to_string(),
                            ref_name: assign_block.local_name,
                            index: index,
                            data: d,
                        };
                    }
                    Err(e) => return Err(e),
                }
            }
        } else if block_name == "TEMPLATE" {
            let template_block = current_route.flow.templates[index].clone();
            let broken_template = template_block.template.split("{}").collect::<Vec<&str>>();
            let mut final_string = String::new();

            let res_condition = match resolve_conditions(
                &template_block.conditions,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(c) => c,
                Err(e) => {
                    return Err(e);
                }
            };

            if res_condition {
                for (i, str) in broken_template.iter().enumerate() {
                    let current_data = if template_block.data.len() == 0 {
                        match RefData::create(false, "STRING", "") {
                            Ok(rd) => rd,
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    } else if template_block.data.len() > i {
                        template_block.data[i].clone()
                    } else {
                        template_block.data[template_block.data.len() - 1].clone()
                    };

                    let current_value = if i == broken_template.len() - 1 {
                        DefinitionData::STRING(String::new())
                    } else {
                        match resolve_ref_data(
                            &current_data,
                            global_blocks,
                            all_definitions,
                            current_index,
                        ) {
                            Ok(d) => d,
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    };

                    match current_value {
                        DefinitionData::STRING(s) => {
                            final_string = format!("{}{}{}", final_string, str, s);
                        }
                        DefinitionData::INTEGER(i) => {
                            final_string = format!("{}{}{}", final_string, str, i);
                        }
                        DefinitionData::FLOAT(f) => {
                            final_string = format!("{}{}{}", final_string, str, f);
                        }
                        DefinitionData::BOOLEAN(b) => {
                            final_string = format!("{}{}{}", final_string, str, b);
                        }
                        DefinitionData::ARRAY(a) => {
                            let mut current_str = String::new();
                            for current in a {
                                match current {
                                    DefinitionData::STRING(s) => {
                                        current_str = format!("{}{}", current_str, s);
                                    }
                                    DefinitionData::INTEGER(i) => {
                                        current_str = format!("{}{}", current_str, i);
                                    }
                                    DefinitionData::FLOAT(f) => {
                                        current_str = format!("{}{}", current_str, f);
                                    }
                                    DefinitionData::BOOLEAN(b) => {
                                        current_str = format!("{}{}", current_str, b);
                                    }
                                    _ => {}
                                }
                            }
                            final_string = format!("{}{}{}", final_string, str, current_str);
                        }
                        _ => {
                            return Err((
                                500,
                                String::from("Error: Invalid data type for template"),
                            ));
                        }
                    }
                }

                actual_definition = DefinitionStore {
                    block_name: block_name.to_string(),
                    ref_name: template_block.local_name,
                    index: index,
                    data: DefinitionData::STRING(final_string),
                };
            }
        } else if block_name == "LOOP" {
            let loop_block = current_route.flow.loops[index].clone();

            let min = match resolve_ref_data(
                &loop_block.min,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(d) => d,
                Err(e) => {
                    return Err(e);
                }
            };

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: loop_block.local_name,
                index: index,
                data: min,
            };
        } else if block_name == "FILTER" {
            let filter_block = current_route.flow.filters[index].clone();
            let current_ref_data = match resolve_raw_data(
                &filter_block.ref_var,
                &global_blocks,
                &all_definitions,
                current_index,
            ) {
                Ok(rd) => rd,
                Err(e) => {
                    return Err(e);
                }
            };

            let current_data = match current_ref_data {
                DefinitionData::ARRAY(a) => a,
                _ => {
                    return Err((
                        500,
                        format!("Error: Invalid data type for '{}'", filter_block.ref_var),
                    ));
                }
            };

            let mut found_property = false;
            let broken_property = filter_block.ref_property.split(".").collect::<Vec<&str>>();
            if broken_property.len() > 2 || broken_property[0].trim().len() < 1 {
                return Err((
                    500,
                    format!(
                        "Error: Invalid property '{}' in filter",
                        filter_block.ref_property
                    ),
                ));
            }

            for raw_pair_data in current_data.iter() {
                let raw_pair = match raw_pair_data {
                    DefinitionData::DATA(d) => d,
                    _ => {
                        continue;
                    }
                };

                if broken_property.len() < 2 {
                    for structure in raw_pair.structures.iter() {
                        if structure.id == broken_property[0] {
                            found_property = true;
                            break;
                        }
                    }
                } else {
                    for custom_structure in raw_pair.custom_structures.iter() {
                        if custom_structure.id == broken_property[0] {
                            for structure in custom_structure.structures.iter() {
                                if structure.id == broken_property[1] {
                                    found_property = true;
                                    break;
                                }
                            }
                        }

                        if found_property {
                            break;
                        }
                    }
                }

                if found_property {
                    break;
                }
            }

            if !found_property {
                return Err((
                    500,
                    format!(
                        "Error: Property '{}' not found in '{}'",
                        filter_block.ref_property, filter_block.ref_var
                    ),
                ));
            }

            let mut final_current_data = Vec::<RawPair>::new();
            let mut current_definition: DefinitionData;

            for raw_pair_data in current_data.iter() {
                let raw_pair = match raw_pair_data {
                    DefinitionData::DATA(d) => d,
                    _ => {
                        continue;
                    }
                };

                if filter_block.filters.len() < 1 {
                    final_current_data.push(raw_pair.clone());
                }

                let mut current_value = String::new();
                if broken_property.len() < 2 {
                    for structure in raw_pair.structures.iter() {
                        if structure.id == broken_property[0] {
                            current_value = structure.value.clone();
                            break;
                        }
                    }
                } else {
                    for custom_structure in raw_pair.custom_structures.iter() {
                        if custom_structure.id == broken_property[0] {
                            for structure in custom_structure.structures.iter() {
                                if structure.id == broken_property[1] {
                                    current_value = structure.value.clone();
                                    break;
                                }
                            }
                        }

                        if current_value.len() > 0 {
                            break;
                        }
                    }
                }

                if current_value.len() < 1 {
                    continue;
                }

                current_definition = DefinitionData::STRING(current_value);
                let mut current_operations: Vec<Operation>;
                let mut current_err: (usize, String) = (0, String::new());

                for filter in filter_block.filters.iter() {
                    let mut rtype = BodyDataType::to(filter.right.rtype.clone());
                    if rtype == String::from("OTHER") {
                        rtype = String::from("STRING");
                    }

                    let data = match current_definition {
                        DefinitionData::STRING(s) => s,
                        DefinitionData::INTEGER(i) => i.to_string(),
                        DefinitionData::FLOAT(f) => f.to_string(),
                        DefinitionData::BOOLEAN(b) => b.to_string(),
                        _ => {
                            current_definition = DefinitionData::UNDEFINED;
                            current_err = (500, String::from("Error: Invalid result data type while processing filter operations"));
                            break;
                        }
                    };

                    match RefData::create(false, &rtype, &data) {
                        Ok(left) => {
                            current_operations = Vec::<Operation>::new();
                            Operation::create(
                                &mut current_operations,
                                left,
                                filter.right.clone(),
                                &OperationType::to(filter.operation_type.clone()),
                                filter.not,
                                &NextConditionType::to(filter.next.clone()),
                            );

                            match resolve_operations(
                                &current_operations,
                                global_blocks,
                                all_definitions,
                                current_index,
                            ) {
                                Ok(d) => {
                                    current_definition = d;
                                }
                                Err(e) => {
                                    current_definition = DefinitionData::UNDEFINED;
                                    current_err = e;
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            current_definition = DefinitionData::UNDEFINED;
                            current_err = e;
                            break;
                        }
                    }
                }

                if current_definition == DefinitionData::UNDEFINED && current_err.0 > 0 {
                    return Err((
                        current_err.0,
                        format!(
                            "Error: Failed processing Filter: '{}'",
                            current_err.1.split("Error: ").collect::<Vec<&str>>()[1]
                        ),
                    ));
                } else {
                    match current_definition {
                        DefinitionData::BOOLEAN(b) => {
                            if b {
                                final_current_data.push(raw_pair.clone());
                            }
                        }
                        _ => {
                            return Err((
                                500,
                                format!("Error: Invalid data type for last operation in Filter"),
                            ));
                        }
                    }
                }
            }

            let mut final_data = Vec::<DefinitionData>::new();
            for raw_pair in final_current_data.iter() {
                final_data.push(DefinitionData::DATA(raw_pair.clone()));
            }

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: filter_block.local_name,
                index: index,
                data: DefinitionData::ARRAY(final_data),
            };
        } else if block_name == "PROPERTY" {
            let property_block = current_route.flow.properties[index].clone();

            let property_data = match resolve_ref_data(
                &property_block.property.data,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(d) => d,
                Err(e) => {
                    return Err(e);
                }
            };

            let final_data: DefinitionData;

            match property_block.property.apply {
                PropertyApply::LENGTH => match property_data {
                    DefinitionData::STRING(s) => {
                        final_data = DefinitionData::INTEGER(s.len() as isize);
                    }
                    DefinitionData::ARRAY(a) => {
                        final_data = DefinitionData::INTEGER(a.len() as isize);
                    }
                    _ => {
                        return Err((500, format!("Error: Invalid data type for LENGTH property")));
                    }
                },
                PropertyApply::GET_FIRST => match property_data {
                    DefinitionData::STRING(s) => {
                        if s.len() > 0 {
                            final_data =
                                DefinitionData::STRING(s.chars().nth(0).unwrap().to_string());
                        } else {
                            return Err((
                                500,
                                format!("Error: Invalid data type for GET_FIRST property"),
                            ));
                        }
                    }
                    DefinitionData::ARRAY(a) => {
                        if a.len() > 0 {
                            final_data = a[0].clone();
                        } else {
                            return Err((
                                500,
                                format!("Error: Invalid data type for GET_FIRST property"),
                            ));
                        }
                    }
                    _ => {
                        return Err((
                            500,
                            format!("Error: Invalid data type for GET_FIRST property"),
                        ));
                    }
                },
                PropertyApply::GET_LAST => match property_data {
                    DefinitionData::STRING(s) => {
                        if s.len() > 0 {
                            final_data = DefinitionData::STRING(
                                s.chars().nth(s.len() - 1).unwrap().to_string(),
                            );
                        } else {
                            return Err((
                                500,
                                format!("Error: Invalid data type for GET_LAST property"),
                            ));
                        }
                    }
                    DefinitionData::ARRAY(a) => {
                        if a.len() > 0 {
                            final_data = a[a.len() - 1].clone();
                        } else {
                            return Err((
                                500,
                                format!("Error: Invalid data type for GET_LAST property"),
                            ));
                        }
                    }
                    _ => {
                        return Err((
                            500,
                            format!("Error: Invalid data type for GET_LAST property"),
                        ));
                    }
                },
                PropertyApply::GET_INDEX => {
                    let mut index = 0;
                    if property_block.property.additional.trim() != "" {
                        match property_block.property.additional.trim().parse::<usize>() {
                            Ok(i) => {
                                index = i;
                            }
                            Err(_) => {
                                return Err((
                                    500,
                                    format!("Error: Invalid value for 'additional' in the GET_INDEX property"),
                                ));
                            }
                        }
                    }

                    match property_data {
                        DefinitionData::STRING(s) => {
                            if s.len() > index {
                                final_data = DefinitionData::STRING(
                                    s.chars().nth(index).unwrap().to_string(),
                                );
                            } else {
                                return Err((
                                    500,
                                    format!("Error: Invalid data type for GET_INDEX property"),
                                ));
                            }
                        }
                        DefinitionData::ARRAY(a) => {
                            if a.len() > index {
                                final_data = a[index].clone();
                            } else {
                                return Err((
                                    500,
                                    format!("Error: Invalid data type for GET_INDEX property"),
                                ));
                            }
                        }
                        _ => {
                            return Err((
                                500,
                                format!("Error: Invalid data type for GET_INDEX property"),
                            ));
                        }
                    }
                }
                PropertyApply::GET_PROPERTY => {
                    let mut property_name = "";
                    if property_block.property.additional.trim() != "" {
                        property_name = &property_block.property.additional.trim();
                    }

                    if property_name.len() < 1 {
                        return Err((
                            500,
                            format!("Error: Invalid value for 'additional' in the GET_PROPERTY property"),
                        ));
                    }

                    match property_data {
                        DefinitionData::ARRAY(a) => {
                            let mut current_value = String::new();
                            let broken_property = property_name.split(".").collect::<Vec<&str>>();
                            if broken_property.len() > 2 || broken_property[0].trim().len() < 1 {
                                return Err((
                                    500,
                                    format!(
                                        "Error: Invalid property '{}' in property",
                                        property_block.property.additional
                                    ),
                                ));
                            }

                            for raw_pair_data in a.iter() {
                                let raw_pair = match raw_pair_data {
                                    DefinitionData::DATA(d) => d,
                                    _ => {
                                        continue;
                                    }
                                };

                                if broken_property.len() < 2 {
                                    for structure in raw_pair.structures.iter() {
                                        if structure.id == broken_property[0] {
                                            current_value = structure.value.clone();
                                            break;
                                        }
                                    }
                                } else {
                                    for custom_structure in raw_pair.custom_structures.iter() {
                                        if custom_structure.id == broken_property[0] {
                                            for structure in custom_structure.structures.iter() {
                                                if structure.id == broken_property[1] {
                                                    current_value = structure.value.clone();
                                                    break;
                                                }
                                            }
                                        }

                                        if current_value.len() > 0 {
                                            break;
                                        }
                                    }
                                }

                                if current_value.len() > 0 {
                                    break;
                                }
                            }

                            if current_value.len() < 1 {
                                return Err((
                                    500,
                                    format!(
                                        "Error: Property '{}' not found in '{}'",
                                        property_block.property.additional,
                                        property_block.property.data.data
                                    ),
                                ));
                            }

                            if let Ok(i) = current_value.parse::<isize>() {
                                final_data = DefinitionData::INTEGER(i);
                            } else if let Ok(f) = current_value.parse::<f64>() {
                                final_data = DefinitionData::FLOAT(f);
                            } else if let Ok(b) = current_value.parse::<bool>() {
                                final_data = DefinitionData::BOOLEAN(b);
                            } else {
                                final_data = DefinitionData::STRING(current_value);
                            }
                        }
                        _ => {
                            return Err((
                                500,
                                format!("Error: Invalid data type for GET_PROPERTY property"),
                            ));
                        }
                    }
                }
            }

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: property_block.local_name,
                index: index,
                data: final_data,
            };
        } else if block_name == "FUNCTION" {
            let function_block = current_route.flow.functions[index].clone();

            let final_data: DefinitionData;

            match function_block.func.id {
                FunctionList::GENERATE_TIMESTAMP => {
                    let timestamp = Utc::now().to_string();
                    final_data = DefinitionData::STRING(timestamp);
                }
                FunctionList::V4 => {
                    let uuid = Uuid::new_v4().to_string();
                    final_data = DefinitionData::STRING(uuid);
                }
                FunctionList::PAGINATE => {
                    if function_block.func.params.len() < 1 {
                        return Err((
                            500,
                            format!("Error: Invalid number of parameters for PAGINATE function"),
                        ));
                    }

                    let data_to_paginate = match resolve_ref_data(
                        &function_block.func.params[0],
                        global_blocks,
                        all_definitions,
                        current_index,
                    ) {
                        Ok(d) => match d {
                            DefinitionData::ARRAY(a) => a,
                            _ => {
                                return Err((
                                    500,
                                    format!("Error: Invalid data type for the 'data' param of the PAGINATE function"),
                                ));
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    };

                    let limit = if function_block.func.params.len() > 1 {
                        match resolve_ref_data(
                            &function_block.func.params[1],
                            global_blocks,
                            all_definitions,
                            current_index,
                        ) {
                            Ok(d) => match d {
                                DefinitionData::INTEGER(i) => i as usize,
                                DefinitionData::FLOAT(f) => f as usize,
                                DefinitionData::STRING(s) => {
                                    if let Ok(i) = s.parse::<usize>() {
                                        i
                                    } else {
                                        return Err((
                                            500,
                                            format!("Error: Invalid data type for the 'limit' param of the PAGINATE function"),
                                        ));
                                    }
                                }
                                _ => {
                                    return Err((
                                        500,
                                        format!("Error: Invalid data type for the 'limit' param of the PAGINATE function"),
                                    ));
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    } else {
                        10
                    };

                    let offset = if function_block.func.params.len() > 2 {
                        match resolve_ref_data(
                            &function_block.func.params[2],
                            global_blocks,
                            all_definitions,
                            current_index,
                        ) {
                            Ok(d) => match d {
                                DefinitionData::INTEGER(i) => i as usize,
                                DefinitionData::FLOAT(f) => f as usize,
                                DefinitionData::STRING(s) => {
                                    if let Ok(i) = s.parse::<usize>() {
                                        i
                                    } else {
                                        return Err((
                                            500,
                                            format!("Error: Invalid data type for the 'offset' param of the PAGINATE function"),
                                        ));
                                    }
                                }
                                _ => {
                                    return Err((
                                        500,
                                        format!("Error: Invalid data type for the 'offset' param of the PAGINATE function"),
                                    ));
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    } else {
                        0
                    };

                    let paginated_data = paginate(data_to_paginate, limit, offset);
                    final_data = DefinitionData::ARRAY(paginated_data);
                }
            }

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: function_block.local_name,
                index: index,
                data: final_data,
            };
        } else if block_name == "OBJECT" {
            let object_block = current_route.flow.objects[index].clone();

            let mut structures = Vec::<StructurePair>::new();
            let mut custom_structures = Vec::<CustomStructurePair>::new();

            for pair in object_block.pairs {
                let current_data = match resolve_ref_data(
                    &pair.data,
                    global_blocks,
                    all_definitions,
                    current_index,
                ) {
                    Ok(d) => d,
                    Err(e) => {
                        return Err(e);
                    }
                };

                let mut processed_data = String::new();

                match current_data {
                    DefinitionData::STRING(s) => {
                        processed_data = s;
                    }
                    DefinitionData::INTEGER(i) => {
                        processed_data = i.to_string();
                    }
                    DefinitionData::FLOAT(f) => {
                        processed_data = f.to_string();
                    }
                    DefinitionData::BOOLEAN(b) => {
                        processed_data = b.to_string();
                    }
                    DefinitionData::ARRAY(a) => {
                        for (i, item) in a.iter().enumerate() {
                            match item {
                                DefinitionData::STRING(s) => {
                                    processed_data = format!(
                                        "{}{}{}",
                                        processed_data,
                                        if i == 0 { "" } else { "," },
                                        s
                                    );
                                }
                                DefinitionData::INTEGER(int) => {
                                    processed_data = format!(
                                        "{}{}{}",
                                        processed_data,
                                        if i == 0 { "" } else { "," },
                                        int
                                    );
                                }
                                DefinitionData::FLOAT(f) => {
                                    processed_data = format!(
                                        "{}{}{}",
                                        processed_data,
                                        if i == 0 { "" } else { "," },
                                        f
                                    );
                                }
                                DefinitionData::BOOLEAN(b) => {
                                    processed_data = format!(
                                        "{}{}{}",
                                        processed_data,
                                        if i == 0 { "" } else { "," },
                                        b
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                    DefinitionData::DATA(d) => {
                        let mut current_structures = Vec::<StructurePair>::new();

                        for structure in d.structures {
                            current_structures.push(StructurePair {
                                id: structure.id,
                                value: structure.value,
                                rtype: structure.rtype,
                            });
                        }

                        for custom_structure in d.custom_structures {
                            for structure in custom_structure.structures {
                                current_structures.push(StructurePair {
                                    id: format!("{}.{}", custom_structure.id, structure.id),
                                    value: structure.value,
                                    rtype: structure.rtype,
                                });
                            }
                        }

                        custom_structures.push(CustomStructurePair {
                            id: pair.id,
                            structures: current_structures,
                        });

                        continue;
                    }
                    DefinitionData::NULL => {}
                    DefinitionData::UNDEFINED => {}
                }

                structures.push(StructurePair {
                    id: pair.id,
                    value: processed_data,
                    rtype: BodyDataType::to(pair.data.rtype),
                });
            }

            actual_definition = DefinitionStore {
                block_name: block_name.to_string(),
                ref_name: object_block.local_name,
                index: index,
                data: DefinitionData::DATA(RawPair {
                    data_id: String::new(),
                    structures: structures,
                    custom_structures: custom_structures,
                    published: false,
                }),
            };
        } else if block_name == "UPDATE" {
            let update_block = current_route.flow.updates[index].clone();
            let res_condition = match resolve_conditions(
                &update_block.conditions,
                global_blocks,
                all_definitions,
                current_index,
            ) {
                Ok(c) => c,
                Err(e) => {
                    return Err(e);
                }
            };

            if res_condition {
                let mappings = auto_fetch_all_mappings();
                let all_data =
                    match auto_fetch_all_data(&mappings, &project_id, &update_block.ref_col) {
                        Ok(d) => d,
                        _ => {
                            return Err((500, String::from("Error: Failed fetching data")));
                        }
                    };

                let all_collections = match auto_fetch_all_collections(&mappings) {
                    Ok(u) => u,
                    _ => {
                        return Err((500, String::from("Error: Failed fetching collections")));
                    }
                };

                let collection =
                    match Collection::get(&all_collections, project_id, &update_block.ref_col) {
                        Ok(p) => p,
                        Err(_) => {
                            return Err((
                                404,
                                String::from("Error: No Collection with this collection_id found"),
                            ));
                        }
                    };

                let current_data = Data::get_all(&all_data, project_id, &update_block.ref_col);
                let mut current_definitions = Vec::<DefinitionData>::new();

                for data in current_data {
                    match convert_to_raw(&data, &collection) {
                        Ok(rp) => {
                            current_definitions.push(DefinitionData::DATA(rp));
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    };
                }

                let mut final_current_data = Vec::<RawPair>::new();
                let mut current_definition: DefinitionData;

                for raw_pair_data in current_definitions.iter() {
                    let mut is_raw_pair_valid = false;

                    let raw_pair = match raw_pair_data {
                        DefinitionData::DATA(d) => d,
                        _ => {
                            continue;
                        }
                    };

                    if update_block.targets.len() < 1 {
                        final_current_data.push(raw_pair.clone());
                    }

                    let mut current_err: (usize, String) = (0, String::new());

                    for target in update_block.targets.iter() {
                        let mut current_value = String::new();
                        let mut current_rtype = BodyDataType::STRING;

                        let broken_property = target.field.split(".").collect::<Vec<&str>>();
                        if broken_property.len() > 2 || broken_property[0].trim().len() < 1 {
                            return Err((
                                500,
                                format!("Error: Invalid property '{}' in update", target.field),
                            ));
                        }

                        if broken_property.len() < 2 {
                            for structure in raw_pair.structures.iter() {
                                if structure.id == broken_property[0] {
                                    current_value = structure.value.clone();
                                    current_rtype = BodyDataType::from(&structure.rtype);
                                    break;
                                }
                            }
                        } else {
                            for custom_structure in raw_pair.custom_structures.iter() {
                                if custom_structure.id == broken_property[0] {
                                    for structure in custom_structure.structures.iter() {
                                        if structure.id == broken_property[1] {
                                            current_value = structure.value.clone();
                                            current_rtype = BodyDataType::from(&structure.rtype);
                                            break;
                                        }
                                    }
                                }

                                if current_value.len() > 0 {
                                    break;
                                }
                            }
                        }

                        match RefData::create(
                            false,
                            &BodyDataType::to(current_rtype.clone()),
                            &current_value,
                        ) {
                            Ok(left) => {
                                let mut current_conditions = Vec::<Condition>::new();
                                for condition in target.conditions.iter() {
                                    Condition::create(
                                        &mut current_conditions,
                                        left.clone(),
                                        condition.right.clone(),
                                        &ConditionType::to(condition.condition_type.clone()),
                                        condition.not,
                                        &NextConditionType::to(condition.next.clone()),
                                    );
                                }

                                match resolve_conditions(
                                    &current_conditions,
                                    global_blocks,
                                    all_definitions,
                                    current_index,
                                ) {
                                    Ok(b) => {
                                        current_definition = DefinitionData::BOOLEAN(b);
                                    }
                                    Err(e) => {
                                        current_definition = DefinitionData::UNDEFINED;
                                        current_err = e;
                                    }
                                }
                            }
                            Err(e) => {
                                current_definition = DefinitionData::UNDEFINED;
                                current_err = e;
                            }
                        }

                        if current_definition == DefinitionData::UNDEFINED && current_err.0 > 0 {
                            return Err((
                                current_err.0,
                                format!(
                                    "Error: Failed processing Update: '{}'",
                                    current_err.1.split("Error: ").collect::<Vec<&str>>()[1]
                                ),
                            ));
                        } else {
                            match current_definition {
                                DefinitionData::BOOLEAN(b) => {
                                    is_raw_pair_valid = b;
                                }
                                _ => {
                                    return Err((
                                500,
                                format!("Error: Invalid data type for last target condition in Update"),
                            ));
                                }
                            }
                        }

                        if !is_raw_pair_valid {
                            break;
                        }
                    }

                    if is_raw_pair_valid {
                        final_current_data.push(raw_pair.clone());
                    }
                }

                let mut final_processed_current_data = Vec::<RawPair>::new();
                let broken_property = update_block.ref_property.split(".").collect::<Vec<&str>>();

                for raw_pair in final_current_data.iter() {
                    if update_block.ref_property.trim().len() < 1 {
                        if let Some(set) = update_block.set.clone() {
                            if let Ok(rd) = resolve_ref_data(
                                &set,
                                global_blocks,
                                all_definitions,
                                current_index,
                            ) {
                                match rd {
                                    DefinitionData::DATA(d) => {
                                        final_processed_current_data.push(d.clone());
                                    }
                                    _ => {
                                        return Err((
                                            500,
                                            format!("Error: Invalid data type for set in Update"),
                                        ));
                                    }
                                }
                            }
                        }
                    } else {
                        let mut current_value = String::new();
                        let mut current_rtype = BodyDataType::STRING;

                        if broken_property.len() < 2 {
                            for structure in raw_pair.structures.iter() {
                                if structure.id == broken_property[0] {
                                    current_value = structure.value.clone();
                                    current_rtype = BodyDataType::from(&structure.rtype);
                                    break;
                                }
                            }
                        } else {
                            for custom_structure in raw_pair.custom_structures.iter() {
                                if custom_structure.id == broken_property[0] {
                                    for structure in custom_structure.structures.iter() {
                                        if structure.id == broken_property[1] {
                                            current_value = structure.value.clone();
                                            current_rtype = BodyDataType::from(&structure.rtype);
                                            break;
                                        }
                                    }
                                }

                                if current_value.len() > 0 {
                                    break;
                                }
                            }
                        }

                        if let Some(set) = update_block.set.clone() {
                            if let Ok(rd) = resolve_ref_data(
                                &set,
                                global_blocks,
                                all_definitions,
                                current_index,
                            ) {
                                match rd {
                                    DefinitionData::STRING(s) => {
                                        current_value = s;
                                    }
                                    DefinitionData::INTEGER(i) => {
                                        current_value = i.to_string();
                                    }
                                    DefinitionData::FLOAT(f) => {
                                        current_value = f.to_string();
                                    }
                                    DefinitionData::BOOLEAN(b) => {
                                        current_value = b.to_string();
                                    }
                                    DefinitionData::ARRAY(a) => {
                                        let mut array_data = String::new();
                                        for (i, val) in a.iter().enumerate() {
                                            match val {
                                                DefinitionData::STRING(s) => {
                                                    if i == 0 {
                                                        array_data = s.clone();
                                                    } else {
                                                        array_data =
                                                            format!("{},{}", array_data, s);
                                                    }
                                                }
                                                DefinitionData::INTEGER(int) => {
                                                    if i == 0 {
                                                        array_data = int.to_string();
                                                    } else {
                                                        array_data = format!(
                                                            "{},{}",
                                                            array_data,
                                                            int.to_string()
                                                        );
                                                    }
                                                }
                                                DefinitionData::FLOAT(f) => {
                                                    if i == 0 {
                                                        array_data = f.to_string();
                                                    } else {
                                                        array_data = format!(
                                                            "{},{}",
                                                            array_data,
                                                            f.to_string()
                                                        );
                                                    }
                                                }
                                                DefinitionData::BOOLEAN(b) => {
                                                    if i == 0 {
                                                        array_data = b.to_string();
                                                    } else {
                                                        array_data = format!(
                                                            "{},{}",
                                                            array_data,
                                                            b.to_string()
                                                        );
                                                    }
                                                }
                                                _ => {
                                                    return Err((
                                                        500,
                                                        format!("Error: Invalid data type for set in Update"),
                                                    ));
                                                }
                                            }
                                        }
                                        current_value = array_data;
                                    }
                                    _ => {
                                        return Err((
                                            500,
                                            format!("Error: Invalid data type for set in Update"),
                                        ));
                                    }
                                }
                            }
                        }

                        if let Some(filter) = update_block.filter.clone() {
                            if current_rtype.clone() == BodyDataType::ARRAY
                                && filter.right.rtype != BodyDataType::ARRAY
                            {
                                let mut final_current_value = String::new();

                                for c_val in current_value.split(",") {
                                    match resolve_operations(
                                        &vec![Operation {
                                            left: RefData {
                                                ref_var: false,
                                                rtype: filter.right.rtype.clone(),
                                                data: c_val.trim().to_string(),
                                            },
                                            right: filter.right.clone(),
                                            operation_type: filter.operation_type.clone(),
                                            next: filter.next.clone(),
                                            not: filter.not,
                                        }],
                                        global_blocks,
                                        all_definitions,
                                        current_index,
                                    ) {
                                        Ok(def) => match def {
                                            DefinitionData::BOOLEAN(b) => {
                                                if b {
                                                    if final_current_value.len() > 0 {
                                                        final_current_value = format!(
                                                            "{},{}",
                                                            final_current_value, c_val
                                                        );
                                                    } else {
                                                        final_current_value = c_val.to_string();
                                                    }
                                                }
                                            }
                                            _ => {
                                                return Err((
                                                        500,
                                                        format!("Error: Invalid data type for filter in Update"),
                                                    ));
                                            }
                                        },
                                        _ => {}
                                    }
                                }

                                current_value = final_current_value;
                            }
                        }

                        if let Some(add) = update_block.add.clone() {
                            if let Ok(rd) = resolve_ref_data(
                                &add,
                                global_blocks,
                                all_definitions,
                                current_index,
                            ) {
                                if current_rtype == BodyDataType::ARRAY {
                                    match rd {
                                        DefinitionData::STRING(s) => {
                                            if current_value.len() > 0 {
                                                current_value = format!("{},{}", current_value, s);
                                            } else {
                                                current_value = s;
                                            }
                                        }
                                        DefinitionData::INTEGER(i) => {
                                            if current_value.len() > 0 {
                                                current_value =
                                                    format!("{},{}", current_value, i.to_string());
                                            } else {
                                                current_value = i.to_string();
                                            }
                                        }
                                        DefinitionData::FLOAT(f) => {
                                            if current_value.len() > 0 {
                                                current_value =
                                                    format!("{},{}", current_value, f.to_string());
                                            } else {
                                                current_value = f.to_string();
                                            }
                                        }
                                        DefinitionData::BOOLEAN(b) => {
                                            if current_value.len() > 0 {
                                                current_value =
                                                    format!("{},{}", current_value, b.to_string());
                                            } else {
                                                current_value = b.to_string();
                                            }
                                        }
                                        DefinitionData::ARRAY(a) => {
                                            let mut array_data = String::new();
                                            for (i, val) in a.iter().enumerate() {
                                                match val {
                                                    DefinitionData::STRING(s) => {
                                                        if i == 0 {
                                                            array_data = s.clone();
                                                        } else {
                                                            array_data =
                                                                format!("{},{}", array_data, s);
                                                        }
                                                    }
                                                    DefinitionData::INTEGER(int) => {
                                                        if i == 0 {
                                                            array_data = int.to_string();
                                                        } else {
                                                            array_data = format!(
                                                                "{},{}",
                                                                array_data,
                                                                int.to_string()
                                                            );
                                                        }
                                                    }
                                                    DefinitionData::FLOAT(f) => {
                                                        if i == 0 {
                                                            array_data = f.to_string();
                                                        } else {
                                                            array_data = format!(
                                                                "{},{}",
                                                                array_data,
                                                                f.to_string()
                                                            );
                                                        }
                                                    }
                                                    DefinitionData::BOOLEAN(b) => {
                                                        if i == 0 {
                                                            array_data = b.to_string();
                                                        } else {
                                                            array_data = format!(
                                                                "{},{}",
                                                                array_data,
                                                                b.to_string()
                                                            );
                                                        }
                                                    }
                                                    _ => {
                                                        return Err((
                                                            500,
                                                            format!("Error: Invalid data type for add in Update"),
                                                        ));
                                                    }
                                                }
                                            }
                                            if current_value.len() > 0 {
                                                current_value =
                                                    format!("{},{}", current_value, array_data);
                                            } else {
                                                current_value = array_data;
                                            }
                                        }
                                        _ => {}
                                    }
                                } else {
                                    match resolve_operations(
                                        &vec![Operation {
                                            left: RefData {
                                                ref_var: false,
                                                rtype: current_rtype,
                                                data: current_value,
                                            },
                                            right: add,
                                            operation_type: OperationType::ADDITION,
                                            next: NextConditionType::NONE,
                                            not: false,
                                        }],
                                        global_blocks,
                                        all_definitions,
                                        current_index,
                                    ) {
                                        Ok(res) => match res {
                                            DefinitionData::STRING(s) => {
                                                current_value = s;
                                            }
                                            DefinitionData::INTEGER(i) => {
                                                current_value = i.to_string();
                                            }
                                            DefinitionData::FLOAT(f) => {
                                                current_value = f.to_string();
                                            }
                                            DefinitionData::BOOLEAN(b) => {
                                                current_value = b.to_string();
                                            }
                                            DefinitionData::ARRAY(a) => {
                                                let mut array_data = String::new();
                                                for (i, val) in a.iter().enumerate() {
                                                    match val {
                                                        DefinitionData::STRING(s) => {
                                                            if i == 0 {
                                                                array_data = s.clone();
                                                            } else {
                                                                array_data =
                                                                    format!("{},{}", array_data, s);
                                                            }
                                                        }
                                                        DefinitionData::INTEGER(int) => {
                                                            if i == 0 {
                                                                array_data = int.to_string();
                                                            } else {
                                                                array_data = format!(
                                                                    "{},{}",
                                                                    array_data,
                                                                    int.to_string()
                                                                );
                                                            }
                                                        }
                                                        DefinitionData::FLOAT(f) => {
                                                            if i == 0 {
                                                                array_data = f.to_string();
                                                            } else {
                                                                array_data = format!(
                                                                    "{},{}",
                                                                    array_data,
                                                                    f.to_string()
                                                                );
                                                            }
                                                        }
                                                        DefinitionData::BOOLEAN(b) => {
                                                            if i == 0 {
                                                                array_data = b.to_string();
                                                            } else {
                                                                array_data = format!(
                                                                    "{},{}",
                                                                    array_data,
                                                                    b.to_string()
                                                                );
                                                            }
                                                        }
                                                        _ => {
                                                            return Err((
                                                                    500,
                                                                    format!("Error: Invalid data type for add in Update"),
                                                                ));
                                                        }
                                                    }
                                                }
                                                current_value = array_data;
                                            }
                                            _ => {
                                                return Err((
                                                        500,
                                                        format!("Error: Invalid data type for add in Update"),
                                                    ));
                                            }
                                        },
                                        Err(e) => {
                                            return Err(e);
                                        }
                                    }
                                }
                            }
                        }

                        let mut final_structures = Vec::<StructurePair>::new();
                        let mut final_custom_structures = Vec::<CustomStructurePair>::new();

                        for structure in raw_pair.structures.iter() {
                            if broken_property.len() < 2 && structure.id == broken_property[0] {
                                final_structures.push(StructurePair {
                                    id: structure.id.clone(),
                                    value: current_value.clone(),
                                    rtype: structure.rtype.clone(),
                                });
                            } else {
                                final_structures.push(structure.clone());
                            }
                        }

                        for custom_structure in raw_pair.custom_structures.iter() {
                            if broken_property.len() > 1
                                && custom_structure.id == broken_property[0]
                            {
                                let mut final_custom_structure_structures =
                                    Vec::<StructurePair>::new();

                                for structure in custom_structure.structures.iter() {
                                    if structure.id == broken_property[1] {
                                        final_custom_structure_structures.push(StructurePair {
                                            id: structure.id.clone(),
                                            value: current_value.clone(),
                                            rtype: structure.rtype.clone(),
                                        });
                                    } else {
                                        final_custom_structure_structures.push(structure.clone());
                                    }
                                }

                                final_custom_structures.push(CustomStructurePair {
                                    id: custom_structure.id.clone(),
                                    structures: final_custom_structure_structures,
                                });
                            } else {
                                final_custom_structures.push(custom_structure.clone());
                            }
                        }

                        final_processed_current_data.push(RawPair {
                            data_id: raw_pair.data_id.clone(),
                            structures: final_structures,
                            custom_structures: final_custom_structures,
                            published: raw_pair.published,
                        });
                    }
                }

                let mut final_data_to_save = Vec::<RawPair>::new();

                let mut all_data_ids = Vec::<String>::new();
                for raw_pair in final_processed_current_data.iter() {
                    if raw_pair.data_id != "" {
                        all_data_ids.push(raw_pair.data_id.clone());
                    }
                }

                let mut current_index = 0;
                for definition in current_definitions {
                    if let DefinitionData::DATA(d) = definition {
                        if all_data_ids.contains(&d.data_id) {
                            if current_index < final_processed_current_data.len() {
                                final_data_to_save
                                    .push(final_processed_current_data[current_index].clone());
                                current_index += 1;
                            }
                        } else {
                            final_data_to_save.push(d.clone());
                        }
                    }
                }

                if update_block.save {
                    let mut final_data_to_save_converted = Vec::<Data>::new();
                    for raw_pair in final_data_to_save.iter() {
                        if let Err(e) = convert_from_raw(
                            &mut final_data_to_save_converted,
                            &collection,
                            raw_pair,
                            true,
                        ) {
                            return Err(e);
                        }
                    }

                    match auto_save_all_data(
                        &mappings,
                        &project_id,
                        &update_block.ref_col,
                        &final_data_to_save_converted,
                    ) {
                        Err(e) => {
                            return Err((500, e));
                        }
                        _ => {}
                    }
                }

                let mut final_data = Vec::<DefinitionData>::new();
                for raw_pair in final_data_to_save.iter() {
                    final_data.push(DefinitionData::DATA(raw_pair.clone()));
                }

                actual_definition = DefinitionStore {
                    block_name: block_name.to_string(),
                    ref_name: format!("update_{}_{}", update_block.ref_col, index),
                    index: index,
                    data: DefinitionData::ARRAY(final_data),
                };
            }
        }

        if current_index >= all_definitions.len() {
            all_definitions.push(actual_definition);
        } else {
            all_definitions[current_index] = actual_definition;
        }

        Ok(())
    }

    pub fn update_definition_value(
        all_definitions: &mut Vec<DefinitionStore>,
        position: usize,
        data: DefinitionData,
    ) -> Result<(), (usize, String)> {
        if position > all_definitions.len() {
            return Err((500, String::from("Error: Invalid position for definition")));
        }

        all_definitions[position].data = data;

        Ok(())
    }

    pub fn get_raw_definition(
        all_definitions: &Vec<DefinitionStore>,
        ref_name: &str,
        index: usize,
    ) -> Option<DefinitionStore> {
        for definition in all_definitions {
            if definition.ref_name == ref_name && definition.index == index {
                return Some(definition.clone());
            }
        }

        None
    }

    pub fn to_string(all_definitions: &Vec<DefinitionStore>) -> Vec<String> {
        let mut definitions = Vec::<String>::new();
        for definition in all_definitions {
            definitions.push(format!(
                "{}/{}: {}",
                definition.block_name, definition.ref_name, definition.index
            ));
        }
        definitions
    }
}
