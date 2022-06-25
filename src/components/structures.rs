use std::fmt;

use rocket::serde::{Deserialize, Serialize};
// use crate::encryption::EncryptionKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    TEXT,
    EMAIL,
    PASSWORD,
    RICHTEXT,
    NUMBER,
    ENUM,
    DATE,
    MEDIA,
    BOOLEAN,
    UID,
    JSON,
    CUSTOM(String),
}

impl Default for Type {
    fn default() -> Self {
        Type::TEXT
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stype_txt = match self {
            Type::TEXT => "text",
            Type::EMAIL => "email",
            Type::PASSWORD => "password",
            Type::RICHTEXT => "richtext",
            Type::NUMBER => "number",
            Type::ENUM => "enum",
            Type::DATE => "date",
            Type::MEDIA => "media",
            Type::BOOLEAN => "bool",
            Type::UID => "uid",
            Type::JSON => "json",
            Type::CUSTOM(s) => &*s,
        };

        write!(f, "{}", stype_txt)
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Structure {
    pub id: String,
    pub name: String,
    pub description: String,
    pub stype: Type,
    pub default_val: String,
    pub min: usize,
    pub max: usize,
    pub encrypted: bool,
    pub unique: bool,
    pub regex_pattern: String,
    pub array: bool,
}

impl Structure {
    pub fn create(
        all_structures: &mut Vec<Structure>,
        id: &str,
        name: &str,
        description: &str,
        stype_txt: &str,
        default_val: &str,
        min: usize,
        max: usize,
        encrypted: bool,
        unique: bool,
        regex_pattern: &str,
        array: bool,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_structure = Structure {
            id: tmp_id.clone(),
            name: "".to_string(),
            description: "".to_string(),
            stype: Type::default(),
            default_val: "".to_string(),
            min: 0,
            max: 0,
            encrypted: false,
            unique: false,
            regex_pattern: "".to_string(),
            array: false,
        };
        all_structures.push(new_structure);

        let id_update = Self::update_id(all_structures, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let name_update = Self::update_name(all_structures, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let description_update = Self::update_description(all_structures, &new_id, description);
            if let Err(e) = description_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let type_update = Self::update_type(all_structures, &new_id, stype_txt);
            if let Err(e) = type_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let default_update = Self::update_default(all_structures, &new_id, default_val);
            if let Err(e) = default_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let min_update = Self::update_min(all_structures, &new_id, min);
            if let Err(e) = min_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let max_update = Self::update_max(all_structures, &new_id, max);
            if let Err(e) = max_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let encrypted_update = Self::update_encrypted(all_structures, &new_id, encrypted);
            if let Err(e) = encrypted_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let unique_update = Self::update_unique(all_structures, &new_id, unique);
            if let Err(e) = unique_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let regex_update = Self::update_regex(all_structures, &new_id, regex_pattern);
            if let Err(e) = regex_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let array_update = Self::update_array(all_structures, &new_id, array);
            if let Err(e) = array_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_structure = Self::delete(all_structures, &new_id);
            if let Err(e) = delete_structure {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn exist(all_structures: &Vec<Structure>, id: &str) -> bool {
        let mut found = false;
        for structure in all_structures.iter() {
            if structure.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn update_id(
        all_structures: &mut Vec<Structure>,
        id: &String,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == new_id {
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

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.id = new_id.trim().to_string();
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_name(
        all_structures: &mut Vec<Structure>,
        id: &String,
        name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

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

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.name = name.trim().to_string();
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_description(
        all_structures: &mut Vec<Structure>,
        id: &String,
        description: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

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

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.description = description.trim().to_string();
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_type(
        all_structures: &mut Vec<Structure>,
        id: &String,
        stype_txt: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        if !String::from(stype_txt)
            .chars()
            .all(|c| c != ';' && c != '@' && c != '>' && c != '#')
        {
            return Err((
                400,
                String::from("Error: stype_txt contains an invalid character"),
            ));
        }

        let stype = match stype_txt {
            "text" => Type::TEXT,
            "email" => Type::EMAIL,
            "password" => Type::PASSWORD,
            "richtext" => Type::RICHTEXT,
            "number" => Type::NUMBER,
            "enum" => Type::ENUM,
            "date" => Type::DATE,
            "media" => Type::MEDIA,
            "bool" => Type::BOOLEAN,
            "uid" => Type::UID,
            "json" => Type::JSON,
            _ => Type::CUSTOM(String::from(stype_txt)),
        };

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.stype = stype;
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_default(
        all_structures: &mut Vec<Structure>,
        id: &String,
        default_val: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        if !String::from(default_val)
            .chars()
            .all(|c| c != ';' && c != '@' && c != '>' && c != '#')
        {
            return Err((
                400,
                String::from("Error: default_val contains an invalid character"),
            ));
        }

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.default_val = String::from(default_val.trim());
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_min(
        all_structures: &mut Vec<Structure>,
        id: &String,
        min: usize,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.min = min;
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_max(
        all_structures: &mut Vec<Structure>,
        id: &String,
        max: usize,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.max = max;
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_encrypted(
        all_structures: &mut Vec<Structure>,
        id: &String,
        encrypted: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.encrypted = encrypted;
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_unique(
        all_structures: &mut Vec<Structure>,
        id: &String,
        unique: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.unique = unique;
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_regex(
        all_structures: &mut Vec<Structure>,
        id: &String,
        regex_pattern: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        if !String::from(regex_pattern)
            .chars()
            .all(|c| c != ';' && c != '@' && c != '>' && c != '#')
        {
            return Err((
                400,
                String::from("Error: regex_pattern contains an invalid character"),
            ));
        }

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.regex_pattern = regex_pattern.trim().to_string();
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn update_array(
        all_structures: &mut Vec<Structure>,
        id: &String,
        array: bool,
    ) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == *id {
                found_structure = Some(structure.clone());
                structure.array = array;
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        Ok(())
    }

    pub fn delete(all_structures: &mut Vec<Structure>, id: &String) -> Result<(), (usize, String)> {
        let mut found_structure: Option<Structure> = None;

        for structure in all_structures.iter_mut() {
            if structure.id == id.to_string() {
                found_structure = Some(structure.clone());
                break;
            }
        }

        if let None = found_structure {
            return Err((404, String::from("Error: Structure not found")));
        }

        let updated_structures: Vec<Structure> = all_structures
            .iter_mut()
            .filter(|structure| structure.id != *id)
            .map(|structure| Structure {
                id: structure.id.clone(),
                name: structure.name.clone(),
                description: structure.description.clone(),
                stype: structure.stype.clone(),
                default_val: structure.default_val.clone(),
                min: structure.min.clone(),
                max: structure.max.clone(),
                encrypted: structure.encrypted.clone(),
                unique: structure.unique.clone(),
                regex_pattern: structure.regex_pattern.clone(),
                array: structure.array.clone(),
            })
            .collect::<Vec<Structure>>();

        *all_structures = updated_structures;

        Ok(())
    }

    pub fn stringify(all_structures: &Vec<Structure>) -> String {
        let mut stringified_structures = String::new();

        for structure in all_structures {
            stringified_structures = format!(
                "{}{}{}",
                stringified_structures,
                if stringified_structures.chars().count() > 1 {
                    "%"
                } else {
                    ""
                },
                Structure::to_string(structure.clone()),
            );
        }

        stringified_structures
    }

    pub fn from_string(structure_str: &str) -> Result<Structure, (usize, String)> {
        let current_structure = structure_str.split("|").collect::<Vec<&str>>();
        let mut tmp_structures = Vec::<Structure>::new();

        if try_add_structure(&current_structure, &mut tmp_structures) {
            return Ok(tmp_structures[0].clone());
        }

        Err((400, String::from("Error: Wrong format for Structure data")))
    }

    pub fn to_string(structure: Structure) -> String {
        let stype_txt = match structure.stype.clone() {
            Type::TEXT => "text".to_string(),
            Type::EMAIL => "email".to_string(),
            Type::PASSWORD => "password".to_string(),
            Type::RICHTEXT => "richtext".to_string(),
            Type::NUMBER => "number".to_string(),
            Type::ENUM => "enum".to_string(),
            Type::DATE => "date".to_string(),
            Type::MEDIA => "media".to_string(),
            Type::BOOLEAN => "bool".to_string(),
            Type::UID => "uid".to_string(),
            Type::JSON => "json".to_string(),
            Type::CUSTOM(txt) => txt.clone(),
        };

        format!(
            "{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
            structure.id,
            structure.name,
            structure.description,
            stype_txt,
            structure.default_val,
            structure.min,
            structure.max,
            structure.encrypted,
            structure.unique,
            structure.regex_pattern,
            structure.array
        )
    }
}

pub fn try_add_structure(array: &Vec<&str>, final_structures: &mut Vec<Structure>) -> bool {
    if array.len() <= 1 {
        return false;
    }

    let min = array[5].parse::<usize>();
    if let Err(e) = min {
        println!("{}", e);
        return false;
    }

    let max = array[6].parse::<usize>();
    if let Err(e) = max {
        println!("{}", e);
        return false;
    }

    let encrypted = match array[7] {
        "true" => true,
        _ => false,
    };

    let unique = match array[8] {
        "true" => true,
        _ => false,
    };

    let is_array = match array[10] {
        "true" => true,
        _ => false,
    };

    let create_structure = Structure::create(
        final_structures,
        array[0],
        array[1],
        array[2],
        array[3],
        array[4],
        min.unwrap(),
        max.unwrap(),
        encrypted,
        unique,
        array[9],
        is_array,
    );

    if let Err(e) = create_structure {
        println!("{}", e.1);
    }

    true
}
