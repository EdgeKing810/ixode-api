use rocket::serde::{Deserialize, Serialize};

use super::sub_ref_data::RefData;

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectPair {
    pub id: String,
    pub data: RefData,
}

impl ObjectPair {
    pub fn create(
        all_pairs: &mut Vec<ObjectPair>,
        id: &str,
        data: RefData,
    ) -> Result<(), (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let tmp_id = "test;";

        let new_pair = ObjectPair {
            id: tmp_id.to_string(),
            data: data,
        };
        all_pairs.push(new_pair);

        if !has_error {
            let id_update = Self::update_id(all_pairs, tmp_id, id);
            if let Err(e) = id_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_pair = Self::delete(all_pairs, tmp_id);
            if let Err(e) = delete_pair {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn update_id(
        all_pairs: &mut Vec<ObjectPair>,
        id: &str,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_pair: Option<ObjectPair> = None;

        if !String::from(new_id)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: new_id contains an invalid character"),
            ));
        }

        if String::from(new_id.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: new_id does not contain enough characters"),
            ));
        } else if String::from(new_id.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: new_id contains too many characters"),
            ));
        }

        for pair in all_pairs.iter_mut() {
            if pair.id == id {
                found_pair = Some(pair.clone());
                pair.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: Object Pair not found")));
        }

        Ok(())
    }

    pub fn update_data(
        all_pairs: &mut Vec<ObjectPair>,
        id: &str,
        data: RefData,
    ) -> Result<(), (usize, String)> {
        let mut found_pair: Option<ObjectPair> = None;

        for pair in all_pairs.iter_mut() {
            if pair.id == id {
                found_pair = Some(pair.clone());
                pair.data = data;
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: Object Pair not found")));
        }

        Ok(())
    }

    pub fn delete(all_pairs: &mut Vec<ObjectPair>, id: &str) -> Result<(), (usize, String)> {
        let mut found_pair: Option<ObjectPair> = None;

        for pair in all_pairs.iter_mut() {
            if pair.id == id {
                found_pair = Some(pair.clone());
                break;
            }
        }

        if let None = found_pair {
            return Err((404, String::from("Error: Object Pair not found")));
        }

        let updated_pairs: Vec<ObjectPair> = all_pairs
            .iter_mut()
            .filter(|pair| pair.id != id)
            .map(|pair| ObjectPair {
                id: pair.id.clone(),
                data: pair.data.clone(),
            })
            .collect::<Vec<ObjectPair>>();

        *all_pairs = updated_pairs;

        Ok(())
    }

    pub fn stringify(all_pairs: &Vec<ObjectPair>) -> String {
        let mut stringified_pairs = String::new();

        for pair in all_pairs {
            stringified_pairs = format!(
                "{}{}{}",
                stringified_pairs,
                if stringified_pairs.chars().count() > 1 {
                    ">"
                } else {
                    ""
                },
                ObjectPair::to_string(pair.clone()),
            );
        }

        stringified_pairs
    }

    pub fn from_string(
        all_pairs: &mut Vec<ObjectPair>,
        pair_str: &str,
    ) -> Result<(), (usize, String)> {
        let mut current_pair = pair_str.split("(").collect::<Vec<&str>>();
        if current_pair.len() <= 1 {
            return Err((500, String::from("Invalid object (at declaration start)")));
        }

        current_pair = current_pair[1].split(")").collect::<Vec<&str>>();
        if current_pair.len() <= 1 {
            return Err((500, String::from("Invalid object (at declaration end)")));
        }

        current_pair = current_pair[0].split("=").collect::<Vec<&str>>();
        if current_pair.len() < 2 {
            return Err((500, String::from("Invalid object (in format)")));
        }

        let id = current_pair[0].trim();
        let data = match RefData::from_string(current_pair[1].trim()) {
            Ok(d) => d,
            Err(e) => return Err((500, format!("Invalid data in object -> {}", e.1))),
        };

        match ObjectPair::create(all_pairs, id, data) {
            Ok(_) => return Ok(()),
            Err(e) => return Err((500, format!("Invalid object (while processing) -> {}", e.1))),
        }
    }

    pub fn to_string(pair: ObjectPair) -> String {
        let data_str = RefData::to_string(pair.data);

        format!("({}={})", pair.id, data_str)
    }
}
