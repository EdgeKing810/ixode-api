use rocket::serde::{Deserialize, Serialize};

use crate::utils::{mapping::auto_fetch_all_mappings, constraint::auto_fetch_all_constraints};

use super::constraint_property::ConstraintProperty;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DataPair {
    pub id: String,
    pub structure_id: String,
    pub custom_structure_id: String,
    pub value: String,
    pub dtype: String,
}

impl DataPair {
    pub fn create(
        all_pairs: &mut Vec<DataPair>,
        id: &str,
        structure_id: &str,
        custom_structure_id: &str,
        value: &str,
        dtype: &str,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_pair = DataPair {
            id: tmp_id.clone(),
            structure_id: "".to_string(),
            custom_structure_id: "".to_string(),
            value: "".to_string(),
            dtype: "".to_string(),
        };
        all_pairs.push(new_pair);

        let id_update = Self::update_id(all_pairs, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_id = tmp_id.clone();
        }

        let structure_id_update = Self::update_structure_id(all_pairs, &new_id, structure_id);
        if let Err(e) = structure_id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        let custom_structure_id_update =
            Self::update_custom_structure_id(all_pairs, &new_id, custom_structure_id);
        if let Err(e) = custom_structure_id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        if !has_error {
            let value_update = Self::update_value(all_pairs, &new_id, value);
            if let Err(e) = value_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let dtype_update = Self::update_dtype(all_pairs, &new_id, dtype);
            if let Err(e) = dtype_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_pair = Self::delete(all_pairs, &new_id);
            if let Err(e) = delete_pair {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_pairs: &Vec<DataPair>, id: &str) -> bool {
        let mut found = false;
        for pair in all_pairs.iter() {
            if pair.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_id(
        all_pairs: &mut Vec<DataPair>,
        id: &String,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_pair: Option<DataPair> = None;

        for pair in all_pairs.iter_mut() {
            if pair.id == *new_id {
                return Err((403, String::from("Error: id is already in use")));
            }
        }

        let final_id = new_id
            .split("----------")
            .collect::<Vec<&str>>()
            .join("---");

            let mappings = auto_fetch_all_mappings();
            let all_constraints = match auto_fetch_all_constraints(&mappings) {
                Ok(c) => c,
                Err(e) => return Err((500, e)),
            };
            let final_value = match ConstraintProperty::validate(&all_constraints, "datapair", "id", &final_id) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.id = final_value;
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: DataPair not found")));
        }

        Ok(())
    }

    pub fn update_structure_id(
        all_pairs: &mut Vec<DataPair>,
        id: &String,
        structure_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_pair: Option<DataPair> = None;

        let final_structure_id = structure_id
            .split("----------")
            .collect::<Vec<&str>>()
            .join("---");

            let mappings = auto_fetch_all_mappings();
            let all_constraints = match auto_fetch_all_constraints(&mappings) {
                Ok(c) => c,
                Err(e) => return Err((500, e)),
            };
            let final_value = match ConstraintProperty::validate(&all_constraints, "datapair", "structure_id", &final_structure_id) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.structure_id = final_value;
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: DataPair not found")));
        }

        Ok(())
    }

    pub fn update_custom_structure_id(
        all_pairs: &mut Vec<DataPair>,
        id: &String,
        custom_structure_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_pair: Option<DataPair> = None;

        let final_custom_structure_id = custom_structure_id
            .split("----------")
            .collect::<Vec<&str>>()
            .join("---");

            let mappings = auto_fetch_all_mappings();
            let all_constraints = match auto_fetch_all_constraints(&mappings) {
                Ok(c) => c,
                Err(e) => return Err((500, e)),
            };
            let final_value = match ConstraintProperty::validate(&all_constraints, "datapair", "custom_structure_id", &final_custom_structure_id) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.custom_structure_id = final_value;
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: DataPair not found")));
        }

        Ok(())
    }

    pub fn update_value(
        all_pairs: &mut Vec<DataPair>,
        id: &String,
        value: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_pair: Option<DataPair> = None;

        let mut final_value = value.split("§").collect::<Vec<&str>>().join("_");

        let mappings = auto_fetch_all_mappings();
            let all_constraints = match auto_fetch_all_constraints(&mappings) {
                Ok(c) => c,
                Err(e) => return Err((500, e)),
            };
            final_value = match ConstraintProperty::validate(&all_constraints, "datapair", "value", &final_value) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.value = final_value;
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: DataPair not found")));
        }

        Ok(())
    }

    pub fn update_dtype(
        all_pairs: &mut Vec<DataPair>,
        id: &String,
        dtype: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_pair: Option<DataPair> = None;

        let final_dtype = dtype.split("----------").collect::<Vec<&str>>().join("---");

        let mappings = auto_fetch_all_mappings();
            let all_constraints = match auto_fetch_all_constraints(&mappings) {
                Ok(c) => c,
                Err(e) => return Err((500, e)),
            };
            let final_value = match ConstraintProperty::validate(&all_constraints, "datapair", "dtype", &final_dtype) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.dtype = final_value;
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: DataPair not found")));
        }

        Ok(())
    }

    pub fn bulk_update_value(
        all_pairs: &mut Vec<DataPair>,
        structure_id: &str,
        value: &str,
    ) -> Result<(), (usize, String)> {
        let mut pair_ids = Vec::<String>::new();
        for pair in all_pairs.iter_mut() {
            if pair.structure_id == structure_id {
                pair_ids.push(pair.id.clone());
            }
        }

        for pair_id in pair_ids {
            match DataPair::update_value(all_pairs, &pair_id, value) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub fn bulk_update_dtype(
        all_pairs: &mut Vec<DataPair>,
        structure_id: &str,
        dtype: &str,
    ) -> Result<(), (usize, String)> {
        let mut pair_ids = Vec::<String>::new();
        for pair in all_pairs.iter_mut() {
            if pair.structure_id == structure_id {
                pair_ids.push(pair.id.clone());
            }
        }

        for pair_id in pair_ids {
            match DataPair::update_dtype(all_pairs, &pair_id, dtype) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub fn bulk_update_structure_id(
        all_pairs: &mut Vec<DataPair>,
        structure_id: &str,
        new_structure_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut pair_ids = Vec::<String>::new();
        for pair in all_pairs.iter_mut() {
            if pair.structure_id == structure_id {
                pair_ids.push(pair.id.clone());
            }
        }

        for pair_id in pair_ids {
            match DataPair::update_structure_id(all_pairs, &pair_id, new_structure_id) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub fn bulk_update_custom_structure_id(
        all_pairs: &mut Vec<DataPair>,
        custom_structure_id: &str,
        new_custom_structure_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut pair_ids = Vec::<String>::new();
        for pair in all_pairs.iter_mut() {
            if pair.custom_structure_id == custom_structure_id {
                pair_ids.push(pair.id.clone());
            }
        }

        for pair_id in pair_ids {
            match DataPair::update_custom_structure_id(all_pairs, &pair_id, new_custom_structure_id)
            {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub fn delete(all_pairs: &mut Vec<DataPair>, id: &String) -> Result<(), (usize, String)> {
        let mut found_pair: Option<DataPair> = None;

        for pair in all_pairs.iter_mut() {
            if pair.id == id.to_string() {
                found_pair = Some(pair.clone());
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: DataPair not found")));
        }

        let updated_pairs: Vec<DataPair> = all_pairs
            .iter_mut()
            .filter(|pair| pair.id != *id)
            .map(|pair| DataPair {
                id: pair.id.clone(),
                structure_id: pair.structure_id.clone(),
                custom_structure_id: pair.custom_structure_id.clone(),
                value: pair.value.clone(),
                dtype: pair.dtype.clone(),
            })
            .collect::<Vec<DataPair>>();

        *all_pairs = updated_pairs;

        Ok(())
    }

    pub fn stringify(pair: DataPair) -> String {
        format!(
            "{}={}={}={}={}",
            pair.id, pair.structure_id, pair.custom_structure_id, pair.dtype, pair.value.split("\n").collect::<Vec<&str>>().join("_newline_"),
        )
    }

    pub fn to_string(all_pairs: &Vec<DataPair>) -> String {
        let mut stringified_pairs = String::new();

        for pair in all_pairs {
            stringified_pairs = format!(
                "{}{}{}",
                stringified_pairs,
                if stringified_pairs.chars().count() > 1 {
                    "§"
                } else {
                    ""
                },
                DataPair::stringify(pair.clone())
            );
        }

        stringified_pairs
    }

    pub fn from_string(all_pairs: &mut Vec<DataPair>, pairs_str: &str) {
        let current_pairs = pairs_str.split("§").collect::<Vec<&str>>();
        for pair in current_pairs {
            let current_pair = pair.split("=").collect::<Vec<&str>>();

            let pair_id = current_pair[0];
            let pair_structure_id = current_pair[1];
            let pair_custom_structure_id = current_pair[2];
            let pair_dtype = current_pair[3];
            let mut pair_value = current_pair[4..].join("=");

            pair_value = pair_value.split("_newline_").collect::<Vec<&str>>().join("\n");

            if let Err(e) = DataPair::create(
                all_pairs,
                pair_id,
                &pair_structure_id,
                &pair_custom_structure_id,
                &pair_value,
                &pair_dtype,
            ) {
                println!("{}", e.1);
                continue;
            }
        }
    }
}
