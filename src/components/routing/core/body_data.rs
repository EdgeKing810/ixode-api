use crate::{
    components::constraint_property::ConstraintProperty,
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

use super::super::submodules::sub_body_data_type::BodyDataType;
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BodyData {
    pub id: String,
    pub bdtype: BodyDataType,
}

impl BodyData {
    pub fn create(
        all_pairs: &mut Vec<BodyData>,
        id: &str,
        bdtype_txt: &str,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_body_data = BodyData {
            id: tmp_id.clone(),
            bdtype: BodyDataType::default(),
        };
        all_pairs.push(new_body_data);

        let id_update = Self::update_id(all_pairs, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let type_update = Self::update_type(all_pairs, &new_id, bdtype_txt);
            if let Err(e) = type_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_body_data = Self::delete(all_pairs, &new_id);
            if let Err(e) = delete_body_data {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_pairs: &Vec<BodyData>, id: &str) -> bool {
        let mut found = false;
        for body_data in all_pairs.iter() {
            if body_data.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_id(
        all_pairs: &mut Vec<BodyData>,
        id: &String,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_body_data: Option<BodyData> = None;

        for body_data in all_pairs.iter_mut() {
            if body_data.id == new_id {
                return Err((403, String::from("Error: id is already in use")));
            }
        }

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "body_data", "id", new_id) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for body_data in all_pairs.iter_mut() {
            if body_data.id == *id {
                found_body_data = Some(body_data.clone());
                body_data.id = final_value;
                break;
            }
        }

        if let None = found_body_data {
            return Err((404, String::from("Error: Body Data not found")));
        }

        Ok(())
    }

    pub fn update_type(
        all_pairs: &mut Vec<BodyData>,
        id: &String,
        bdtype_txt: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_body_data: Option<BodyData> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "body_data", "bdtype", bdtype_txt)
            {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        let bdtype = BodyDataType::from(&final_value);

        for body_data in all_pairs.iter_mut() {
            if body_data.id == *id {
                found_body_data = Some(body_data.clone());
                body_data.bdtype = bdtype;
                break;
            }
        }

        if let None = found_body_data {
            return Err((404, String::from("Error: Body Data not found")));
        }

        Ok(())
    }

    pub fn delete(all_pairs: &mut Vec<BodyData>, id: &String) -> Result<(), (usize, String)> {
        let mut found_body_data: Option<BodyData> = None;

        for body_data in all_pairs.iter_mut() {
            if body_data.id == id.to_string() {
                found_body_data = Some(body_data.clone());
                break;
            }
        }

        if let None = found_body_data {
            return Err((404, String::from("Error: Body Data not found")));
        }

        let updated_pairs: Vec<BodyData> = all_pairs
            .iter_mut()
            .filter(|body_data| body_data.id != *id)
            .map(|body_data| BodyData {
                id: body_data.id.clone(),
                bdtype: body_data.bdtype.clone(),
            })
            .collect::<Vec<BodyData>>();

        *all_pairs = updated_pairs;

        Ok(())
    }

    pub fn stringify(all_pairs: &Vec<BodyData>, is_param: bool) -> String {
        let mut stringified_body_data = String::new();

        for body_data in all_pairs {
            stringified_body_data = format!(
                "{}{}{}",
                stringified_body_data,
                if stringified_body_data.chars().count() > 1 {
                    "\n"
                } else {
                    ""
                },
                BodyData::to_string(body_data.clone(), is_param),
            );
        }

        stringified_body_data
    }

    pub fn from_string(
        all_pairs: &mut Vec<BodyData>,
        body_data_str: &str,
        is_param: bool,
    ) -> Result<(), (usize, String)> {
        let pre_string = if is_param {
            "ADD PARAMS pair"
        } else {
            "ADD BODY pair"
        };

        let mut current_body_data_obj = body_data_str
            .split(&format!("{} [", pre_string))
            .collect::<Vec<&str>>();
        if current_body_data_obj.len() <= 1 {
            return Err((
                500,
                String::from("Invalid body_data (at declaration start)"),
            ));
        }

        current_body_data_obj = current_body_data_obj[1].split("]").collect::<Vec<&str>>();
        if current_body_data_obj.len() <= 1 {
            return Err((500, String::from("Invalid body_data (at declaration end)")));
        }

        current_body_data_obj = current_body_data_obj[0].split(",").collect::<Vec<&str>>();
        if current_body_data_obj.len() < 2 {
            return Err((500, String::from("Invalid body_data (in format)")));
        }

        BodyData::create(
            all_pairs,
            current_body_data_obj[0],
            current_body_data_obj[1],
        )
    }

    pub fn to_string(body_data: BodyData, is_param: bool) -> String {
        let bdtype_txt = BodyDataType::to(body_data.bdtype);

        let pre_string = if is_param {
            "ADD PARAMS pair"
        } else {
            "ADD BODY pair"
        };

        format!("{} [{},{}]", pre_string, body_data.id, bdtype_txt)
    }
}
