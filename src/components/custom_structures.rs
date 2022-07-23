use crate::components::structures::{try_add_structure, Structure};
use rocket::serde::{Deserialize, Serialize};
// use crate::encryption::EncryptionKey;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct CustomStructure {
    pub id: String,
    pub name: String,
    pub description: String,
    pub structures: Vec<Structure>,
}

impl CustomStructure {
    pub fn create(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &str,
        name: &str,
        description: &str,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_custom_structure = CustomStructure {
            id: tmp_id.clone(),
            name: "".to_string(),
            description: "".to_string(),
            structures: vec![],
        };
        all_custom_structures.push(new_custom_structure);

        let id_update = Self::update_id(all_custom_structures, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let name_update = Self::update_name(all_custom_structures, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let description_update =
                Self::update_description(all_custom_structures, &new_id, description);
            if let Err(e) = description_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_custom_structure = Self::delete(all_custom_structures, &new_id);
            if let Err(e) = delete_custom_structure {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_custom_structures: &Vec<CustomStructure>, id: &str) -> bool {
        let mut found = false;
        for custom_structure in all_custom_structures.iter() {
            if custom_structure.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_id(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == new_id {
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
                String::from("Error: new_id does not contain enough characters"),
            ));
        } else if String::from(new_id.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: new_id contains too many characters"),
            ));
        }

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                custom_structure.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        Ok(())
    }

    pub fn update_name(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        if !String::from(name)
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ' ')
        {
            return Err((
                400,
                String::from("Error: name contains an invalid character"),
            ));
        }

        if String::from(name.trim()).len() < 1 {
            return Err((
                400,
                String::from("Error: name does not contain enough characters"),
            ));
        } else if String::from(name.trim()).len() > 100 {
            return Err((
                400,
                String::from("Error: name contains too many characters"),
            ));
        }

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                custom_structure.name = name.trim().to_string();
                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        Ok(())
    }

    pub fn update_description(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        description: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        if !String::from(description)
            .chars()
            .all(|c| c != ';' && c != '@' && c != '>' && c != '#')
        {
            return Err((
                400,
                String::from("Error: description contains an invalid character"),
            ));
        }

        if String::from(description.trim()).len() > 1000 {
            return Err((
                400,
                String::from("Error: description contains too many characters"),
            ));
        }

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                custom_structure.description = description.trim().to_string();
                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        Ok(())
    }

    pub fn add_structure(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structure: Structure,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());

                let mut current_structures = custom_structure.structures.clone();

                match Structure::create(
                    &mut current_structures,
                    &structure.id,
                    &structure.name,
                    &structure.description,
                    &structure.stype.to_string(),
                    &structure.default_val,
                    structure.min,
                    structure.max,
                    structure.encrypted,
                    structure.unique,
                    &structure.regex_pattern,
                    structure.array,
                    structure.required,
                ) {
                    Err(e) => return Err(e),
                    _ => {}
                }

                custom_structure.structures = current_structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        Ok(())
    }

    pub fn update_structure(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structure_id: &String,
        structure: Structure,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());

                let mut current_structures = custom_structure.structures.clone();
                let mut updated_structures = Vec::<Structure>::new();

                for current_structure in current_structures.iter_mut() {
                    if current_structure.id != *structure_id {
                        updated_structures.push(current_structure.clone());
                    } else {
                        match Structure::create(
                            &mut updated_structures,
                            &structure.id,
                            &structure.name,
                            &structure.description,
                            &structure.stype.to_string(),
                            &structure.default_val,
                            structure.min,
                            structure.max,
                            structure.encrypted,
                            structure.unique,
                            &structure.regex_pattern,
                            structure.array,
                            structure.required,
                        ) {
                            Err(e) => return Err(e),
                            _ => {}
                        }
                    }
                }

                custom_structure.structures = updated_structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        Ok(())
    }

    pub fn set_structures(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structures: Vec<Structure>,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());
                custom_structure.structures = structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        Ok(())
    }

    pub fn remove_structure(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
        structure_id: &String,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == *id {
                found_custom_structure = Some(custom_structure.clone());

                let mut current_structures = custom_structure.structures.clone();
                match Structure::delete(&mut current_structures, structure_id) {
                    Err(e) => return Err(e),
                    _ => {}
                }

                custom_structure.structures = current_structures;

                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        Ok(())
    }

    pub fn delete(
        all_custom_structures: &mut Vec<CustomStructure>,
        id: &String,
    ) -> Result<(), (usize, String)> {
        let mut found_custom_structure: Option<CustomStructure> = None;

        for custom_structure in all_custom_structures.iter_mut() {
            if custom_structure.id == id.to_string() {
                found_custom_structure = Some(custom_structure.clone());
                break;
            }
        }

        if let None = found_custom_structure {
            return Err((404, String::from("Error: Custom Structure not found")));
        }

        let updated_structures: Vec<CustomStructure> = all_custom_structures
            .iter_mut()
            .filter(|custom_structure| custom_structure.id != *id)
            .map(|custom_structure| CustomStructure {
                id: custom_structure.id.clone(),
                name: custom_structure.name.clone(),
                description: custom_structure.description.clone(),
                structures: custom_structure.structures.clone(),
            })
            .collect::<Vec<CustomStructure>>();

        *all_custom_structures = updated_structures;

        Ok(())
    }

    pub fn stringify(all_custom_structures: &Vec<CustomStructure>) -> String {
        let mut stringified_custom_structures = String::new();

        for custom_structure in all_custom_structures {
            stringified_custom_structures = format!(
                "{}{}{}",
                stringified_custom_structures,
                if stringified_custom_structures.chars().count() > 1 {
                    "#"
                } else {
                    ""
                },
                CustomStructure::to_string(custom_structure.clone()),
            );
        }

        stringified_custom_structures
    }

    pub fn from_string(custom_structure_str: &str) -> Result<CustomStructure, (usize, String)> {
        let current_custom_structure = custom_structure_str.split("|").collect::<Vec<&str>>();

        if current_custom_structure.len() < 3 {
            return Err((
                400,
                String::from("Error: Wrong format for Custom Structure data"),
            ));
        }

        let mut tmp_custom_structures = Vec::<CustomStructure>::new();

        let create_custom_structure = CustomStructure::create(
            &mut tmp_custom_structures,
            current_custom_structure[0],
            current_custom_structure[1],
            current_custom_structure[2],
        );

        if let Err(e) = create_custom_structure {
            return Err(e);
        }

        let mut tmp_structures = Vec::<Structure>::new();
        let current_structures = current_custom_structure[3..].join("|");
        let individual_structures = current_structures.split("%").collect::<Vec<&str>>();

        for structure in individual_structures {
            let current_structure = structure.split("|").collect::<Vec<&str>>();

            if !try_add_structure(&current_structure, &mut tmp_structures) {
                continue;
            }
        }

        Ok(tmp_custom_structures[0].clone())
    }

    pub fn to_string(custom_structure: CustomStructure) -> String {
        let stringified_structures = Structure::stringify(&custom_structure.structures);

        format!(
            "{}|{}|{}|{}",
            custom_structure.id,
            custom_structure.name,
            custom_structure.description,
            stringified_structures
        )
    }
}
