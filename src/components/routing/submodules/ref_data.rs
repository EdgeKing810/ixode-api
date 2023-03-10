use rocket::serde::{Deserialize, Serialize};

use crate::{
    components::constraint_property::ConstraintProperty,
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

use super::sub_body_data_type::BodyDataType;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefData {
    pub ref_var: bool,
    pub rtype: BodyDataType,
    pub data: String,
}

impl Default for RefData {
    fn default() -> Self {
        RefData {
            ref_var: false,
            rtype: BodyDataType::default(),
            data: String::new(),
        }
    }
}

impl RefData {
    pub fn create(ref_var: bool, rtype: &str, data: &str) -> Result<RefData, (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let mut ref_var_obj = RefData {
            ref_var: ref_var,
            rtype: BodyDataType::from(rtype),
            data: "".to_string(),
        };

        let data_update = Self::update_data(&mut ref_var_obj, data);
        if let Err(e) = data_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        if has_error {
            return Err(latest_error);
        }

        Ok(ref_var_obj)
    }

    pub fn update_data(ref_var_obj: &mut RefData, data: &str) -> Result<(), (usize, String)> {
        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "ref_data", "data", data) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        ref_var_obj.data = final_value;

        Ok(())
    }

    pub fn to_string(ref_data_obj: RefData) -> String {
        format!(
            "[{},{},{}]",
            if ref_data_obj.ref_var == true {
                "ref"
            } else {
                ""
            },
            BodyDataType::to(ref_data_obj.rtype),
            ref_data_obj.data
        )
    }

    pub fn from_string(ref_data_obj_str: &str) -> Result<RefData, (usize, String)> {
        let mut current_data_obj_str = ref_data_obj_str.split("[").collect::<Vec<&str>>();
        if current_data_obj_str.len() <= 1 {
            return Err((500, String::from("Invalid ref_data (at declaration start)")));
        }

        current_data_obj_str = current_data_obj_str[1].split("]").collect::<Vec<&str>>();
        if current_data_obj_str.len() <= 1 {
            return Err((500, String::from("Invalid ref_data (at declaration end)")));
        }

        current_data_obj_str = current_data_obj_str[0].split(",").collect::<Vec<&str>>();
        if current_data_obj_str.len() < 3 {
            return Err((500, String::from("Invalid ref_data (in format)")));
        }

        let ref_data = RefData {
            ref_var: current_data_obj_str[0] == "ref",
            rtype: BodyDataType::from(current_data_obj_str[1]),
            data: current_data_obj_str[2].to_string(),
        };

        Ok(ref_data)
    }
}
