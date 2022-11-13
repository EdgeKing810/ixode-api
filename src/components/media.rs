use crate::{components::io::{fetch_file, save_file}, utils::{mapping::auto_fetch_all_mappings, constraint::auto_fetch_all_constraints}};
use rocket::serde::{Deserialize, Serialize};

use super::constraint_property::ConstraintProperty;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Media {
    pub id: String,
    name: String,
}

impl Media {
    fn create_no_check(id: &str, name: &str) -> Media {
        Media {
            id: String::from(id),
            name: String::from(name),
        }
    }

    pub fn exist(all_medias: &Vec<Media>, id: &str) -> bool {
        let mut found = false;
        for media in all_medias.iter() {
            if media.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn get(all_medias: &Vec<Media>, media_id: &str) -> Result<Media, (usize, String)> {
        for media in all_medias.iter() {
            if media.id.to_lowercase() == media_id.to_lowercase() {
                return Ok(media.clone());
            }
        }

        Err((404, String::from("Error: Media not found")))
    }

    pub fn create(
        all_medias: &mut Vec<Media>,
        id: &str,
        name: &str,
    ) -> Result<(), (usize, String)> {
        let tmp_id = String::from("test;");
        let mut new_id = String::from(id);

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let new_media = Media {
            id: tmp_id.clone(),
            name: "".to_string(),
        };
        all_medias.push(new_media);

        let id_update = Self::update_id(all_medias, &tmp_id, id);
        if let Err(e) = id_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
            new_id = tmp_id;
        }

        if !has_error {
            let name_update = Self::update_name(all_medias, &new_id, name);
            if let Err(e) = name_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_media = Self::delete(all_medias, &new_id);
            if let Err(e) = delete_media {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(())
    }

    pub fn update_id(
        all_medias: &mut Vec<Media>,
        id: &String,
        new_id: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_media: Option<Media> = None;

        for media in all_medias.iter() {
            if media.id == new_id {
                return Err((403, String::from("Error: id is already in use")));
            }
        }

        let mappings = auto_fetch_all_mappings();
            let all_constraints = match auto_fetch_all_constraints(&mappings) {
                Ok(c) => c,
                Err(e) => return Err((500, e)),
            };
            let final_value = match ConstraintProperty::validate(&all_constraints, "media", "id", new_id) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for media in all_medias.iter_mut() {
            if media.id == *id {
                found_media = Some(media.clone());
                media.id = final_value;
                break;
            }
        }

        if let None = found_media {
            return Err((404, String::from("Error: Media not found")));
        }

        Ok(())
    }

    pub fn update_name(
        all_medias: &mut Vec<Media>,
        id: &String,
        name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_media: Option<Media> = None;

        let mappings = auto_fetch_all_mappings();
            let all_constraints = match auto_fetch_all_constraints(&mappings) {
                Ok(c) => c,
                Err(e) => return Err((500, e)),
            };
            let final_value = match ConstraintProperty::validate(&all_constraints, "media", "name", name) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for media in all_medias.iter_mut() {
            if media.id == *id {
                found_media = Some(media.clone());
                media.name = final_value;
                break;
            }
        }

        if let None = found_media {
            return Err((404, String::from("Error: Media not found")));
        }

        Ok(())
    }

    pub fn delete(all_medias: &mut Vec<Media>, id: &String) -> Result<(), (usize, String)> {
        let mut found_media: Option<Media> = None;

        for media in all_medias.iter_mut() {
            if media.id == id.to_string() {
                found_media = Some(media.clone());
                break;
            }
        }

        if let None = found_media {
            return Err((404, String::from("Error: Media not found")));
        }

        let updated_medias: Vec<Media> = all_medias
            .iter_mut()
            .filter(|media| media.id != *id)
            .map(|media| Media {
                id: media.id.clone(),
                name: media.name.clone(),
            })
            .collect::<Vec<Media>>();

        *all_medias = updated_medias;

        Ok(())
    }

    pub fn to_string(media: Media) -> String {
        format!("{}^{}", media.id, media.name)
    }

    pub fn from_string(media_str: &str) -> Media {
        let current_media = media_str.split("^").collect::<Vec<&str>>();

        Media::create_no_check(current_media[0], current_media[1])
    }
}

pub fn stringify_medias(medias: &Vec<Media>) -> String {
    let mut stringified_medias = String::new();

    for media in medias {
        stringified_medias = format!(
            "{}{}{}",
            stringified_medias,
            if stringified_medias.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            Media::to_string(media.clone()),
        );
    }

    stringified_medias
}

pub fn unwrap_medias(all_medias_raw: String) -> Vec<Media> {
    let individual_medias = all_medias_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_medias: Vec<Media> = Vec::<Media>::new();

    for media in individual_medias {
        let tmp_media = Media::from_string(media);
        final_medias.push(tmp_media);
    }

    final_medias
}

pub fn fetch_all_medias(path: String, encryption_key: &String) -> Vec<Media> {
    let all_medias_raw = fetch_file(path.clone(), encryption_key);
    let final_medias: Vec<Media> = unwrap_medias(all_medias_raw);
    final_medias
}

pub fn save_all_medias(medias: &Vec<Media>, path: String, encryption_key: &String) {
    let stringified_medias = stringify_medias(medias);
    save_file(path, stringified_medias, encryption_key);
    println!("Medias saved!");
}
