use crate::routes::x_utils::definition_store::DefinitionData;

pub fn definition_to_string(
    value: DefinitionData,
    str: &str,
    category: &str,
    ignore_error: bool,
) -> Result<String, (usize, String)> {
    let mut final_string = String::new();

    match value {
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
            for (i, current) in a.iter().enumerate() {
                match definition_to_string(current.clone(), "", category, ignore_error) {
                    Ok(s) => {
                        current_str = format!(
                            "{}{}{}",
                            current_str,
                            if i == 0 || current_str.trim().len() <= 0 || s.trim().len() <= 0 {
                                ""
                            } else {
                                ","
                            },
                            s.trim()
                        );
                    }
                    Err(e) => {
                        if !ignore_error {
                            return Err(e);
                        }
                    }
                }
            }
            final_string = format!("{}{}{}", final_string, str, current_str);
        }
        _ => {
            if !ignore_error {
                return Err((500, format!("Error: Invalid data type for {}", category)));
            }
        }
    }

    Ok(final_string)
}
