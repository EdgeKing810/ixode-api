use crate::components::datapair::DataPair;
use crate::components::io::{fetch_file, save_file};
// use crate::encryption::{EncryptionKey};
use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub id: String,
    pub project_id: String,
    pub collection_id: String,
    pub pairs: Vec<DataPair>,
    pub published: bool,
}

impl Data {
    pub fn create(
        all_data: &mut Vec<Data>,
        id: &str,
        project_id: &str,
        collection_id: &str,
        published: bool,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_data = Data {
            id: tmp_id.clone(),
            project_id: "".to_string(),
            collection_id: "".to_string(),
            pairs: vec![],
            published: false,
        };
        all_data.push(new_data);

        let id_update = Self::update_id(all_data, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_id = tmp_id.clone();
        }

        if !has_error {
            let project_id_update = Self::update_project_id(all_data, &new_id, project_id);
            if let Err(e) = project_id_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let collection_id_update = Self::update_collection_id(all_data, &new_id, collection_id);
            if let Err(e) = collection_id_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let published_update = Self::update_published(all_data, &new_id, published);
            if let Err(e) = published_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_data = Self::delete(all_data, &new_id);
            if let Err(e) = delete_data {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_data: &Vec<Data>, id: &str) -> bool {
        let mut found = false;
        for data in all_data.iter() {
            if data.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get_all(all_data: &Vec<Data>, project_id: &str, collection_id: &str) -> Vec<Data> {
        let mut new_data = Vec::<Data>::new();
        for data in all_data.iter() {
            if data.project_id.to_lowercase() == project_id.to_lowercase()
                && data.collection_id.to_lowercase() == collection_id.to_lowercase()
            {
                new_data.push(data.clone());
            }
        }

        new_data
    }

    pub fn get(
        all_data: &Vec<Data>,
        project_id: &str,
        collection_id: &str,
        id: &str,
    ) -> Result<Data, (usize, String)> {
        for data in all_data.iter() {
            if data.id.to_lowercase() == id.to_lowercase()
                && data.project_id.to_lowercase() == project_id.to_lowercase()
                && data.collection_id.to_lowercase() == collection_id.to_lowercase()
            {
                return Ok(data.clone());
            }
        }

        Err((404, String::from("Error: Data not found")))
    }

    pub fn update_id(
        all_data: &mut Vec<Data>,
        id: &String,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        for data in all_data.iter_mut() {
            if data.id == *new_id {
                return Err((403, String::from("Error: id is already in use")));
            }
        }

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
                String::from("Error: id does not contain enough characters"),
            ));
        } else if String::from(new_id.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: new_id contains too many characters"),
            ));
        }

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());
                data.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        Ok(())
    }

    pub fn update_project_id(
        all_data: &mut Vec<Data>,
        id: &String,
        project_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        if !String::from(project_id)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: project_id contains an invalid character"),
            ));
        }

        if String::from(project_id.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: project_id does not contain enough characters"),
            ));
        } else if String::from(project_id.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: project_id contains too many characters"),
            ));
        }

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());
                data.project_id = project_id.trim().to_string();
                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        Ok(())
    }

    pub fn update_collection_id(
        all_data: &mut Vec<Data>,
        id: &String,
        collection_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        if !String::from(collection_id)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: collection_id contains an invalid character"),
            ));
        }

        if String::from(collection_id.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: collection_id does not contain enough characters"),
            ));
        } else if String::from(collection_id.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: collection_id contains too many characters"),
            ));
        }

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());
                data.collection_id = collection_id.trim().to_string();
                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        Ok(())
    }

    pub fn update_published(
        all_data: &mut Vec<Data>,
        id: &String,
        published: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());
                data.published = published;
                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        Ok(())
    }

    pub fn bulk_update_project_id(
        all_data: &mut Vec<Data>,
        project_id: &str,
        new_project_id: &str,
    ) {
        for data in all_data.iter_mut() {
            if data.project_id == *project_id {
                data.project_id = new_project_id.to_string();
            }
        }
    }

    pub fn bulk_update_collection_id(
        all_data: &mut Vec<Data>,
        collection_id: &str,
        new_collection_id: &str,
    ) {
        for data in all_data.iter_mut() {
            if data.collection_id == *collection_id {
                data.collection_id = new_collection_id.to_string();
            }
        }
    }

    pub fn bulk_update_structure_id(
        all_data: &mut Vec<Data>,
        project_id: &str,
        collection_id: &str,
        structure_id: &str,
        new_structure_id: &str,
    ) -> Result<(), (usize, String)> {
        for data in all_data.iter_mut() {
            if data.project_id == *project_id && data.collection_id == *collection_id {
                let mut current_pairs = data.pairs.clone();

                match DataPair::bulk_update_structure_id(
                    &mut current_pairs,
                    structure_id,
                    new_structure_id,
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                }

                data.pairs = current_pairs;
            }
        }

        Ok(())
    }

    pub fn bulk_update_custom_structure_id(
        all_data: &mut Vec<Data>,
        project_id: &str,
        collection_id: &str,
        custom_structure_id: &str,
        new_custom_structure_id: &str,
    ) -> Result<(), (usize, String)> {
        for data in all_data.iter_mut() {
            if data.project_id == *project_id && data.collection_id == *collection_id {
                let mut current_pairs = data.pairs.clone();

                match DataPair::bulk_update_custom_structure_id(
                    &mut current_pairs,
                    custom_structure_id,
                    new_custom_structure_id,
                ) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                data.pairs = current_pairs;
            }
        }

        Ok(())
    }

    pub fn bulk_update_value(
        all_data: &mut Vec<Data>,
        project_id: &str,
        collection_id: &str,
        structure_id: &str,
        value: &str,
    ) -> Result<(), (usize, String)> {
        for data in all_data.iter_mut() {
            if data.project_id == *project_id && data.collection_id == *collection_id {
                let mut current_pairs = data.pairs.clone();

                match DataPair::bulk_update_value(&mut current_pairs, structure_id, value) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                data.pairs = current_pairs;
            }
        }

        Ok(())
    }

    pub fn bulk_update_stype(
        all_data: &mut Vec<Data>,
        project_id: &str,
        collection_id: &str,
        structure_id: &str,
        stype: &str,
    ) -> Result<(), (usize, String)> {
        for data in all_data.iter_mut() {
            if data.project_id == *project_id && data.collection_id == *collection_id {
                let mut current_pairs = data.pairs.clone();

                match DataPair::bulk_update_dtype(&mut current_pairs, structure_id, stype) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }

                data.pairs = current_pairs;
            }
        }

        Ok(())
    }

    pub fn add_pair(
        all_data: &mut Vec<Data>,
        id: &String,
        pair: DataPair,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());

                let mut current_pairs = data.pairs.clone();

                match DataPair::create(
                    &mut current_pairs,
                    &pair.id,
                    &pair.structure_id,
                    &pair.custom_structure_id,
                    &pair.value,
                    &pair.dtype,
                ) {
                    Err(e) => return Err(e),
                    _ => {}
                }

                data.pairs = current_pairs;

                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        Ok(())
    }

    pub fn update_pair(
        all_data: &mut Vec<Data>,
        id: &String,
        pair_id: &String,
        pair: DataPair,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());

                let mut current_pairs = data.pairs.clone();
                let mut updated_pairs = Vec::<DataPair>::new();

                for current_pair in current_pairs.iter_mut() {
                    if current_pair.id != *pair_id {
                        updated_pairs.push(current_pair.clone());
                    } else {
                        match DataPair::create(
                            &mut updated_pairs,
                            &pair.id,
                            &pair.structure_id,
                            &pair.custom_structure_id,
                            &pair.value,
                            &pair.dtype,
                        ) {
                            Err(e) => return Err(e),
                            _ => {}
                        }
                    }
                }

                data.pairs = updated_pairs;

                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Collection not found")));
        }

        Ok(())
    }

    pub fn set_pairs(
        all_data: &mut Vec<Data>,
        id: &String,
        pairs: Vec<DataPair>,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());
                data.pairs = pairs;

                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        Ok(())
    }

    pub fn remove_pair(
        all_data: &mut Vec<Data>,
        id: &String,
        pair_id: &String,
    ) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        for data in all_data.iter_mut() {
            if data.id == *id {
                found_data = Some(data.clone());

                let mut current_pairs = data.pairs.clone();
                match DataPair::delete(&mut current_pairs, pair_id) {
                    Err(e) => return Err(e),
                    _ => {}
                }

                data.pairs = current_pairs;

                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        Ok(())
    }

    pub fn delete(all_data: &mut Vec<Data>, id: &String) -> Result<(), (usize, String)> {
        let mut found_data: Option<Data> = None;

        for data in all_data.iter_mut() {
            if data.id == id.to_string() {
                found_data = Some(data.clone());
                break;
            }
        }

        if let None = found_data {
            return Err((404, String::from("Error: Data not found")));
        }

        let updated_data: Vec<Data> = all_data
            .iter_mut()
            .filter(|data| data.id != *id)
            .map(|data| Data {
                id: data.id.clone(),
                project_id: data.project_id.clone(),
                collection_id: data.collection_id.clone(),
                pairs: data.pairs.clone(),
                published: data.published,
            })
            .collect::<Vec<Data>>();

        *all_data = updated_data;

        Ok(())
    }

    pub fn delete_by_project(all_data: &mut Vec<Data>, project_id: &String) {
        let updated_data: Vec<Data> = all_data
            .iter_mut()
            .filter(|data| data.project_id != *project_id)
            .map(|data| Data {
                id: data.id.clone(),
                project_id: data.project_id.clone(),
                collection_id: data.collection_id.clone(),
                pairs: data.pairs.clone(),
                published: data.published,
            })
            .collect::<Vec<Data>>();

        *all_data = updated_data;
    }

    pub fn delete_by_collection(all_data: &mut Vec<Data>, collection_id: &String) {
        let updated_data: Vec<Data> = all_data
            .iter_mut()
            .filter(|data| data.collection_id != *collection_id)
            .map(|data| Data {
                id: data.id.clone(),
                project_id: data.project_id.clone(),
                collection_id: data.collection_id.clone(),
                pairs: data.pairs.clone(),
                published: data.published,
            })
            .collect::<Vec<Data>>();

        *all_data = updated_data;
    }

    pub fn to_string(data: Data) -> String {
        let stringified_pairs = DataPair::to_string(&data.pairs);
        let publish_num = if data.published { "1" } else { "0" };

        format!(
            "{};{};{};{};{}",
            data.id, data.project_id, data.collection_id, publish_num, stringified_pairs,
        )
    }

    pub fn from_string(mut all_data: &mut Vec<Data>, data_str: &str) -> String {
        let current_data = data_str.split(";").collect::<Vec<&str>>();
        let published = if current_data[3] == "1" { true } else { false };
        println!("current data: {:#?}", current_data);

        let data_id = current_data[0];
        let create_data = Data::create(
            &mut all_data,
            current_data[0],
            current_data[1],
            current_data[2],
            published,
        );
        if let Err(e) = create_data {
            return e.1;
        }

        let current_pairs = current_data[4..].join(";");
        let mut final_pairs: Vec<DataPair> = vec![];
        DataPair::from_string(&mut final_pairs, &current_pairs);

        let set_pairs = Data::set_pairs(&mut all_data, &data_id.to_string(), final_pairs);
        if let Err(e) = set_pairs {
            return e.1;
        }

        String::new()
    }
}

pub fn stringify_data(all_data: &Vec<Data>) -> String {
    let mut stringified_data = String::new();

    for data in all_data {
        stringified_data = format!(
            "{}{}{}",
            stringified_data,
            if stringified_data.chars().count() > 1 {
                "----------"
            } else {
                ""
            },
            Data::to_string(data.clone())
        );
    }

    stringified_data
}

pub fn unwrap_data(all_data_raw: String) -> Vec<Data> {
    let individual_data = all_data_raw
        .split("----------")
        .filter(|line| line.chars().count() >= 3);

    let mut final_data: Vec<Data> = Vec::<Data>::new();

    for data in individual_data {
        Data::from_string(&mut final_data, data);
    }

    final_data
}

pub fn fetch_all_data(path: String, encryption_key: &String) -> Vec<Data> {
    let all_data_raw = fetch_file(path.clone(), encryption_key);
    let final_data = unwrap_data(all_data_raw);
    final_data
}

pub fn save_all_data(all_data: &Vec<Data>, path: String, encryption_key: &String) {
    let stringified_data = stringify_data(all_data);
    save_file(path, stringified_data, encryption_key);
    println!("Data saved!");
}
