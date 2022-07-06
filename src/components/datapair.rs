use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct DataPair {
    pub id: String,
    pub value: String,
    pub dtype: String,
}

impl DataPair {
    pub fn create(
        all_pairs: &mut Vec<DataPair>,
        id: &str,
        value: &str,
        dtype: &str,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_pair = DataPair {
            id: tmp_id.clone(),
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

        let final_id = new_id.split("----------").collect::<Vec<&str>>().join("---");

        if !final_id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: new_id contains an invalid character"),
            ));
        }

        if final_id.trim().len() < 1 {
            return Err((
                400,
                String::from("Error: id does not contain enough characters"),
            ));
        } else if final_id.trim().len() > 100 {
            return Err((
                400,
                String::from("Error: new_id contains too many characters"),
            ));
        }

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.id = final_id.trim().to_string();
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

        let final_value = value.split("§").collect::<Vec<&str>>().join("_");

        if String::from(final_value.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: value does not contain enough characters"),
            ));
        } else if String::from(final_value.trim()).len() > 50000 {
            return Err((
                400,
                String::from("Error: value contains too many characters"),
            ));
        }

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.value = final_value.trim().to_string();
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

        if final_dtype
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: dtype contains an invalid character"),
            ));
        }

        if final_dtype.trim().len() < 1 {
            return Err((
                400,
                String::from("Error: dtype does not contain enough characters"),
            ));
        } else if final_dtype.trim().len() > 50000 {
            return Err((
                400,
                String::from("Error: dtype contains too many characters"),
            ));
        }

        for pair in all_pairs.iter_mut() {
            if pair.id == *id {
                found_pair = Some(pair.clone());
                pair.dtype = final_dtype.trim().to_string();
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: DataPair not found")));
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
                value: pair.value.clone(),
                dtype: pair.dtype.clone(),
            })
            .collect::<Vec<DataPair>>();

        *all_pairs = updated_pairs;

        Ok(())
    }

    pub fn stringify(pair: DataPair) -> String {
        format!("{}={}={}", pair.id, pair.dtype, pair.value,)
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
            let pair_dtype = current_pair[1];
            let pair_value = current_pair[2..].join("=");

            if let Err(e) = DataPair::create(all_pairs, pair_id, &pair_value, &pair_dtype) {
                println!("{}", e.1);
                continue;
            }
        }
    }
}
