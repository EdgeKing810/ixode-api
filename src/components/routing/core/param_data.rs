use rocket::serde::{Deserialize, Serialize};

use crate::{
    components::constraint_property::ConstraintProperty,
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

use super::core_body_data::BodyData;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParamData {
    pub delimiter: String,
    pub pairs: Vec<BodyData>,
}

impl ParamData {
    pub fn create(delimiter: &str) -> Result<ParamData, (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let mut param_data = ParamData {
            delimiter: "".to_string(),
            pairs: vec![],
        };

        let delimiter_update = Self::update_delimiter(&mut param_data, delimiter);
        if let Err(e) = delimiter_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        if has_error {
            return Err(latest_error);
        }

        Ok(param_data)
    }

    pub fn update_delimiter(
        param_data: &mut ParamData,
        delimiter: &str,
    ) -> Result<(), (usize, String)> {
        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "param_data",
            "delimiter",
            delimiter,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        param_data.delimiter = final_value;

        Ok(())
    }

    pub fn set_body_data_pairs(param_data: &mut ParamData, all_pairs: Vec<BodyData>) {
        param_data.pairs = all_pairs;
    }

    pub fn add_body_data(
        param_data: &mut ParamData,
        id: &str,
        bdtype_txt: &str,
    ) -> Result<(), (usize, String)> {
        let mut all_pairs = param_data.pairs.clone();

        if let Err(e) = BodyData::create(&mut all_pairs, id, bdtype_txt) {
            return Err(e);
        }

        param_data.pairs = all_pairs;

        Ok(())
    }

    pub fn update_body_data_id(
        param_data: &mut ParamData,
        id: &str,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut all_pairs = param_data.pairs.clone();

        if let Err(e) = BodyData::update_id(&mut all_pairs, &id.to_string(), new_id) {
            return Err(e);
        }

        param_data.pairs = all_pairs;

        Ok(())
    }

    pub fn update_body_data_bdtype(
        param_data: &mut ParamData,
        id: &str,
        bdtype_txt: &str,
    ) -> Result<(), (usize, String)> {
        let mut all_pairs = param_data.pairs.clone();

        if let Err(e) = BodyData::update_type(&mut all_pairs, &id.to_string(), bdtype_txt) {
            return Err(e);
        }

        param_data.pairs = all_pairs;

        Ok(())
    }

    pub fn remove_body_data(param_data: &mut ParamData, id: &str) -> Result<(), (usize, String)> {
        let mut all_pairs = param_data.pairs.clone();

        if let Err(e) = BodyData::delete(&mut all_pairs, &id.to_string()) {
            return Err(e);
        }

        param_data.pairs = all_pairs;

        Ok(())
    }

    pub fn to_string(param_data: ParamData) -> String {
        let all_pairs = param_data.pairs.clone();

        format!(
            "DEFINE PARAMS delimiter {}\n{}",
            param_data.delimiter,
            BodyData::stringify(&all_pairs, true)
        )
    }

    pub fn from_string(param_data_str: &str) -> Result<ParamData, (usize, String)> {
        let current_param_data = param_data_str.split("\n").collect::<Vec<&str>>();

        let mut delimiter = "";
        let mut all_pairs = Vec::<BodyData>::new();

        for line in current_param_data.clone() {
            if line.trim().len() > 0 {
                if line.starts_with("DEFINE PARAMS") {
                    let current_delimiter = line
                        .split("DEFINE PARAMS delimiter ")
                        .collect::<Vec<&str>>();

                    if current_delimiter.len() <= 1 {
                        return Err((
                            500,
                            String::from("Invalid param_data (at declaration start)"),
                        ));
                    }

                    delimiter = current_delimiter[1];
                } else if line.starts_with("ADD PARAMS") {
                    if let Err(e) = BodyData::from_string(&mut all_pairs, line, true) {
                        return Err((500, format!("Invalid param in param_data -> {}", e.1)));
                    }
                }
            }
        }

        let mut param_data = match ParamData::create(delimiter) {
            Ok(p) => p,
            Err(e) => {
                return Err((
                    500,
                    format!("Invalid param_data (while processing) ->  {}", e.1),
                ))
            }
        };

        ParamData::set_body_data_pairs(&mut param_data, all_pairs);

        Ok(param_data)
    }
}
