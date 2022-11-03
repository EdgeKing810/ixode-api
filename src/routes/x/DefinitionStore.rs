use rocket::serde::{Deserialize, Serialize};

use crate::components::collection::Collection;
use crate::components::data::Data;
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
use crate::data_converter::{convert_to_raw, RawPair};
use crate::routes::x::LocalParamData;
use crate::utils::{auto_fetch_all_collections, auto_fetch_all_data, auto_fetch_all_mappings};

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
