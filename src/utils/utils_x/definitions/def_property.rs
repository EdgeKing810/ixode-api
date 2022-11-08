use crate::{
    components::routing::{
        blocks::property_block::PropertyBlock, submodules::sub_property_apply::PropertyApply,
    },
    utils::x::{
        definition_store::{DefinitionData, DefinitionStore},
        global_block_order::GlobalBlockOrder,
        resolver::resolve_ref_data,
    },
};

pub fn define_property(
    property_block: PropertyBlock,
    global_blocks: &Vec<GlobalBlockOrder>,
    all_definitions: &mut Vec<DefinitionStore>,
    current_index: usize,
) -> Result<DefinitionData, (usize, String)> {
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
                    final_data = DefinitionData::STRING(s.chars().nth(0).unwrap().to_string());
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
                    final_data =
                        DefinitionData::STRING(s.chars().nth(s.len() - 1).unwrap().to_string());
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
                            format!(
                                "Error: Invalid value for 'additional' in the GET_INDEX property"
                            ),
                        ));
                    }
                }
            }

            match property_data {
                DefinitionData::STRING(s) => {
                    if s.len() > index {
                        final_data =
                            DefinitionData::STRING(s.chars().nth(index).unwrap().to_string());
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
                DefinitionData::DATA(d) => {
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

                    if broken_property.len() < 2 {
                        for structure in d.structures.iter() {
                            if structure.id == broken_property[0] {
                                current_value = structure.value.clone();
                                break;
                            }
                        }
                    } else {
                        for custom_structure in d.custom_structures.iter() {
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

    Ok(final_data)
}
