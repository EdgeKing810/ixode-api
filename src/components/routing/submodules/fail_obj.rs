use rocket::serde::{Deserialize, Serialize};

use crate::{utils::{mapping::auto_fetch_all_mappings, constraint::auto_fetch_all_constraints}, components::constraint_property::ConstraintProperty};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FailObj {
    pub status: u32,
    pub message: String,
}

impl FailObj {
    pub fn create(status: u32, message: &str) -> Result<FailObj, (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let mut fail_obj = FailObj {
            status: status,
            message: "".to_string(),
        };

        let message_update = Self::update_message(&mut fail_obj, message);
        if let Err(e) = message_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        if has_error {
            return Err(latest_error);
        }

        Ok(fail_obj)
    }

    pub fn update_message(fail_obj: &mut FailObj, message: &str) -> Result<(), (usize, String)> {
        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(&all_constraints, "fail_obj", "message", message) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        fail_obj.message = final_value;

        Ok(())
    }

    pub fn to_string(fail_obj: FailObj) -> String {
        format!("[{},{}]", fail_obj.status, fail_obj.message.split("\n").collect::<Vec<&str>>().join("_newline_"))
    }

    pub fn from_string(fail_obj_str: &str) -> Result<FailObj, (usize, String)> {
        let mut current_fail_obj = fail_obj_str.split("[").collect::<Vec<&str>>();
        if current_fail_obj.len() <= 1 {
            return Err((500, String::from("Invalid fail_obj (at declaration start)")));
        }

        current_fail_obj = current_fail_obj[1].split("]").collect::<Vec<&str>>();
        if current_fail_obj.len() <= 1 {
            return Err((500, String::from("Invalid fail_obj (at declaration end)")));
        }

        current_fail_obj = current_fail_obj[0].split(",").collect::<Vec<&str>>();
        if current_fail_obj.len() < 2 {
            return Err((500, String::from("Invalid fail_obj (in format)")));
        }

        let status = match current_fail_obj[0].trim().parse::<u32>() {
            Ok(sts) => sts,
            Err(e) => {
                return Err((
                    500,
                    format!("Invalid fail_obj (in 'status' format) -> {}", e),
                ))
            }
        };

        match FailObj::create(status, &current_fail_obj[1].split("_newline_").collect::<Vec<&str>>().join("\n")) {
            Ok(fail_obj) => Ok(fail_obj),
            Err(e) => {
                return Err((
                    500,
                    format!("Invalid fail_obj (while processing) -> {}", e.1),
                ))
            }
        }
    }
}
