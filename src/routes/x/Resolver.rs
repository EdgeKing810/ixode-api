use crate::components::routing::submodules::sub_body_data_type::BodyDataType;
use crate::components::routing::submodules::sub_condition::Condition;
use crate::components::routing::submodules::sub_condition_type::ConditionType;
use crate::components::routing::submodules::sub_next_condition_type::NextConditionType;
use crate::components::routing::submodules::sub_operation::Operation;
use crate::components::routing::submodules::sub_operation_type::OperationType;
use crate::components::routing::submodules::sub_ref_data::RefData;

use super::definition_store::{DefinitionData, DefinitionStore};
use super::global_block_order::GlobalBlockOrder;

pub fn resolve_ref_data(
    rdata: &RefData,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
    let mut value = DefinitionData::STRING(rdata.data.clone());

    if rdata.ref_var {
        match GlobalBlockOrder::get_ref_index(global_blocks, &rdata.data, current_index) {
            Ok(ri) => {
                let current_definition =
                    DefinitionStore::get_raw_definition(all_definitions, &rdata.data, ri.0);

                if let Some(def) = current_definition {
                    value = def.data.clone();
                } else {
                    value = DefinitionData::NULL;
                }
            }
            Err(_) => value = DefinitionData::UNDEFINED,
        }
    }

    if value == DefinitionData::UNDEFINED {
        return Err((500, format!("Error: '{}' is undefined", rdata.data)));
    }

    match rdata.rtype {
        BodyDataType::BOOLEAN => match value {
            DefinitionData::BOOLEAN(_) => {
                return Ok(value);
            }
            DefinitionData::STRING(s) => {
                if s.trim().to_lowercase() == "true" {
                    return Ok(DefinitionData::BOOLEAN(true));
                }
                return Ok(DefinitionData::BOOLEAN(false));
            }
            DefinitionData::INTEGER(i) => {
                if i >= 1 {
                    return Ok(DefinitionData::BOOLEAN(true));
                }
                return Ok(DefinitionData::BOOLEAN(false));
            }
            DefinitionData::FLOAT(f) => {
                if f >= 1.0 {
                    return Ok(DefinitionData::BOOLEAN(true));
                }
                return Ok(DefinitionData::BOOLEAN(false));
            }
            DefinitionData::NULL => {
                return Ok(DefinitionData::BOOLEAN(false));
            }
            _ => {
                return Err((500, format!("Error: '{}' is not a boolean", rdata.data)));
            }
        },
        BodyDataType::FLOAT => match value {
            DefinitionData::BOOLEAN(b) => {
                if b {
                    return Ok(DefinitionData::FLOAT(1.0));
                }
                return Ok(DefinitionData::FLOAT(0.0));
            }
            DefinitionData::STRING(s) => {
                if let Ok(f) = s.parse::<f64>() {
                    return Ok(DefinitionData::FLOAT(f));
                }
                return Err((500, format!("Error: '{}' is not a float", rdata.data)));
            }
            DefinitionData::INTEGER(i) => {
                return Ok(DefinitionData::FLOAT(i as f64));
            }
            DefinitionData::FLOAT(_) => {
                return Ok(value);
            }
            DefinitionData::NULL => {
                return Ok(DefinitionData::FLOAT(0.0));
            }
            _ => {
                return Err((500, format!("Error: '{}' is not a boolean", rdata.data)));
            }
        },
        BodyDataType::INTEGER => match value {
            DefinitionData::BOOLEAN(b) => {
                if b {
                    return Ok(DefinitionData::INTEGER(1));
                }
                return Ok(DefinitionData::INTEGER(0));
            }
            DefinitionData::STRING(s) => {
                if let Ok(i) = s.parse::<isize>() {
                    return Ok(DefinitionData::INTEGER(i));
                }
                return Err((500, format!("Error: '{}' is not an integer", rdata.data)));
            }
            DefinitionData::INTEGER(_) => {
                return Ok(value);
            }
            DefinitionData::FLOAT(f) => {
                return Ok(DefinitionData::INTEGER(f as isize));
            }
            DefinitionData::NULL => {
                return Ok(DefinitionData::INTEGER(0));
            }
            _ => {
                return Err((500, format!("Error: '{}' is not an integer", rdata.data)));
            }
        },
        BodyDataType::OTHER => match value {
            DefinitionData::DATA(_) => {
                return Ok(value);
            }
            _ => {
                return Err((
                    500,
                    format!("Error: '{}' is not an abnormal data type", rdata.data),
                ));
            }
        },
        BodyDataType::STRING => match value {
            DefinitionData::BOOLEAN(b) => {
                if b {
                    return Ok(DefinitionData::STRING("true".to_string()));
                }
                return Ok(DefinitionData::STRING("false".to_string()));
            }
            DefinitionData::STRING(_) => {
                return Ok(value);
            }
            DefinitionData::INTEGER(i) => {
                return Ok(DefinitionData::STRING(i.to_string()));
            }
            DefinitionData::FLOAT(f) => {
                return Ok(DefinitionData::STRING(f.to_string()));
            }
            DefinitionData::NULL => {
                return Ok(DefinitionData::STRING("".to_string()));
            }
            _ => {
                return Err((500, format!("Error: '{}' is not a string", rdata.data)));
            }
        },
    }
}

pub fn resolve_conditions(
    conditions: &Vec<Condition>,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &Vec<DefinitionStore>,
    current_index: usize,
) -> Result<bool, (usize, String)> {
    if conditions.len() == 0 {
        return Ok(true);
    }

    let mut next = NextConditionType::NONE;
    let mut current_eval = false;

    for condition in conditions {
        let left = match resolve_ref_data(
            &condition.left,
            global_blocks,
            all_definitions,
            current_index,
        ) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let right = match resolve_ref_data(
            &condition.right,
            global_blocks,
            all_definitions,
            current_index,
        ) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let mut local_eval: bool;

        match condition.condition_type {
            ConditionType::EQUAL_TO => {
                local_eval = left == right;
            }
            ConditionType::NOT_EQUAL_TO => {
                local_eval = left != right;
            }
            ConditionType::GREATER_THAN => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l > r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l > r;
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l as f64 > r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l > r as f64;
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l.len() > r as usize;
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = l as usize > r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l.len() > r as usize;
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = l as usize > r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = l.len() > r.len();
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '>'",
                            condition.left.data, condition.right.data
                        ),
                    ));
                }
            },
            ConditionType::GREATER_THAN_OR_EQUAL_TO => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l >= r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l >= r;
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l as f64 >= r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l >= r as f64;
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l.len() >= r as usize;
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = l as usize >= r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l.len() >= r as usize;
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = l as usize >= r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = l.len() >= r.len();
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '>='",
                            condition.left.data, condition.right.data
                        ),
                    ));
                }
            },
            ConditionType::LESS_THAN => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l < r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l < r;
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = (l as f64) < r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l < r as f64;
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l.len() < r as usize;
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = (l as usize) < r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l.len() < r as usize;
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = (l as usize) < r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = l.len() < r.len();
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '<'",
                            condition.left.data, condition.right.data
                        ),
                    ));
                }
            },
            ConditionType::LESS_THAN_OR_EQUAL_TO => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l <= r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l <= r;
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l as f64 <= r;
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l <= r as f64;
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = l.len() <= r as usize;
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = (l as usize) <= r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = l.len() <= r as usize;
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = (l as usize) <= r.len();
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = l.len() <= r.len();
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '<='",
                            condition.left.data, condition.right.data
                        ),
                    ));
                }
            },
            ConditionType::INCLUDES => match (left, right) {
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = l.contains(&r);
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with 'includes'",
                            condition.left.data, condition.right.data
                        ),
                    ));
                }
            },
        }

        if condition.not {
            local_eval = !local_eval;
        }

        if next == NextConditionType::NONE {
            current_eval = local_eval;
        } else if next == NextConditionType::AND {
            current_eval = current_eval && local_eval;
        } else if next == NextConditionType::OR {
            current_eval = current_eval || local_eval;
        }

        next = condition.next.clone();
    }

    Ok(current_eval)
}

pub fn resolve_operations(
    operations: &Vec<Operation>,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
    let mut next = NextConditionType::NONE;
    let mut current_eval = DefinitionData::NULL;

    for operation in operations {
        let left = match resolve_ref_data(
            &operation.left,
            global_blocks,
            all_definitions,
            current_index,
        ) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let right = match resolve_ref_data(
            &operation.right,
            global_blocks,
            all_definitions,
            current_index,
        ) {
            Ok(d) => d,
            Err(e) => return Err(e),
        };

        let mut local_eval;

        match operation.operation_type {
            OperationType::EQUAL_TO => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l == r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l == r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as f64) == r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l == r as f64);
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() == r as usize);
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) == r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() == r as usize);
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) == r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l == r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l == r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l == (r != 0));
                }
                (DefinitionData::INTEGER(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l != 0) == r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l == (r != 0.0));
                }
                (DefinitionData::FLOAT(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l != 0.0) == r);
                }
                (DefinitionData::STRING(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l.trim().to_lowercase() == "true") && r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l && (r.trim().to_lowercase() == "true"));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '=='",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::NOT_EQUAL_TO => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as f64) != r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != r as f64);
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() != r as usize);
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) != r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() != r as usize);
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) != r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != (r != 0));
                }
                (DefinitionData::INTEGER(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l != 0) != r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != (r != 0.0));
                }
                (DefinitionData::FLOAT(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l != 0.0) != r);
                }
                (DefinitionData::STRING(l), DefinitionData::BOOLEAN(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l.trim().to_lowercase() == "true") != r);
                }
                (DefinitionData::BOOLEAN(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l != (r.trim().to_lowercase() == "true"));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '!='",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::GREATER_THAN => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l > r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l > r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as f64) > r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l > r as f64);
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() > r as usize);
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) > r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() > r as usize);
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) > r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l > r);
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '>'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::GREATER_THAN_OR_EQUAL_TO => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l >= r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l >= r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as f64) >= r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l >= r as f64);
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() >= r as usize);
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) >= r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() >= r as usize);
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) >= r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l >= r);
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '>='",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::LESS_THAN => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l < r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l < r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as f64) < r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l < r as f64);
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() < r as usize);
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) < r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() < r as usize);
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) < r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l < r);
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '<'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::LESS_THAN_OR_EQUAL_TO => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l <= r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l <= r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as f64) <= r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l <= r as f64);
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() <= r as usize);
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) <= r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.len() <= r as usize);
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN((l as usize) <= r.len());
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l <= r);
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot compare '{}' and '{}' with '<='",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::ADDITION => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::INTEGER(l + r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT(l + r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT((l as f64) + r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::FLOAT(l + (r as f64));
                }
                (DefinitionData::STRING(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::STRING(format!("{}{}", l, r));
                }
                (DefinitionData::INTEGER(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::STRING(format!("{}{}", l, r));
                }
                (DefinitionData::STRING(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::STRING(format!("{}{}", l, r));
                }
                (DefinitionData::FLOAT(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::STRING(format!("{}{}", l, r));
                }
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::STRING(format!("{}{}", l, r));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot add '{}' and '{}' with '+'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::SUBSTRACTION => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::INTEGER(l - r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT(l - r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT((l as f64) - r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::FLOAT(l - (r as f64));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot substract '{}' and '{}' with '-'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::MULTIPLICATION => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::INTEGER(l * r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT(l * r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT((l as f64) * r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::FLOAT(l * (r as f64));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot multiply '{}' and '{}' with '*'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::DIVISION => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::INTEGER(l / r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT(l / r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT((l as f64) / r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::FLOAT(l / (r as f64));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot divide '{}' and '{}' with '/'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::MODULO => match (left, right) {
                (DefinitionData::INTEGER(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::INTEGER(l % r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT(l % r);
                }
                (DefinitionData::INTEGER(l), DefinitionData::FLOAT(r)) => {
                    local_eval = DefinitionData::FLOAT((l as f64) % r);
                }
                (DefinitionData::FLOAT(l), DefinitionData::INTEGER(r)) => {
                    local_eval = DefinitionData::FLOAT(l % (r as f64));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot modulo '{}' and '{}' with '%'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::INCLUDES => match (left, right) {
                (DefinitionData::STRING(l), DefinitionData::STRING(r)) => {
                    local_eval = DefinitionData::BOOLEAN(l.contains(&r));
                }
                _ => {
                    return Err((
                        500,
                        format!(
                            "Error: Cannot check if '{}' contains '{}' with 'includes'",
                            operation.left.data, operation.right.data
                        ),
                    ));
                }
            },
            OperationType::NONE => {
                if left != DefinitionData::NULL && left != DefinitionData::UNDEFINED {
                    local_eval = left;
                } else {
                    local_eval = right;
                }
            }
        }

        match local_eval {
            DefinitionData::BOOLEAN(b) => {
                if operation.not {
                    local_eval = DefinitionData::BOOLEAN(!b);
                }

                if next == NextConditionType::NONE {
                    current_eval = local_eval;
                } else {
                    match current_eval {
                        DefinitionData::BOOLEAN(mb) => {
                            if next == NextConditionType::AND {
                                current_eval = DefinitionData::BOOLEAN(b && mb);
                            } else if next == NextConditionType::OR {
                                current_eval = DefinitionData::BOOLEAN(b || mb);
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                current_eval = local_eval;
            }
        }

        next = operation.next.clone();
    }

    Ok(current_eval)
}
