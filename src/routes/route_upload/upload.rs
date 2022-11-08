use rocket::http::ContentType;
use rocket::serde::json::{json, Value};
use rocket::{post, Data};

use std::fs::copy;

use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

use crate::components::encryption::EncryptionKey;
use crate::components::media::Media;
use crate::utils::{
    mapping::auto_fetch_all_mappings, media::auto_fetch_all_medias, media::auto_save_all_medias,
};

#[post("/", data = "<data>")]
pub async fn main(content_type: &ContentType, data: Data<'_>) -> Value {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("file")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        // MultipartFormDataField::raw("fingerprint").size_limit(4096),
        // MultipartFormDataField::text("name"),
        // MultipartFormDataField::text("email").repetition(Repetition::fixed(3)),
        // MultipartFormDataField::text("email"),
    ]);

    let multipart_form_data = match MultipartFormData::parse(content_type, data, options).await {
        Ok(mfd) => mfd,
        Err(_) => return json!({"status": 400, "message": "Error: Invalid data submitted"}),
    };

    let media = multipart_form_data.files.get("file");

    if let Some(file_fields) = media {
        let file_field = &file_fields[0];

        let content_type = match &file_field.content_type {
            Some(ct) => ct.to_string(),
            None => return json!({"status": 500, "message": "Error: Could not read Content Type"}),
        };
        let path = &file_field.path;

        if &content_type[..6] != "image/" {
            return json!({"status": 403, "message": format!("Error: Only image files are allowed [{}]", content_type)});
        }

        let file_name = match &file_field.file_name {
            Some(name) => name,
            None => return json!({"status": 500, "message": "Error: Could not read file name"}),
        };
        if file_name.len() > 200 {
            return json!({"status": 403, "message": "Error: File name is too long"});
        }

        let splitted_file_name = match &file_field.file_name {
            Some(name) => name.trim().clone().split(".").collect::<Vec<&str>>(),
            None => return json!({"status": 500, "message": "Error: Could not read file name"}),
        };

        if splitted_file_name.len() <= 1 {
            return json!({"status": 403, "message": "Error: File lacks an extension"});
        }

        let extension = splitted_file_name[splitted_file_name.len() - 1];
        let mut clean_file_name = String::new();
        for i in 0..(splitted_file_name.len() - 1) {
            clean_file_name = format!(
                "{}{}{}",
                clean_file_name,
                if i == 0 { "" } else { "." },
                splitted_file_name[i]
            );
        }

        let splitted_clean_file_name = clean_file_name
            .trim()
            .clone()
            .split(" ")
            .collect::<Vec<&str>>();

        let mut clean_file_name = String::new();
        for i in 0..(splitted_clean_file_name.len()) {
            clean_file_name = format!(
                "{}{}{}",
                clean_file_name,
                if i == 0 { "" } else { "_" },
                splitted_clean_file_name[i]
            );
        }

        let splitted_respect_file_name = clean_file_name
            .trim()
            .clone()
            .split("^")
            .collect::<Vec<&str>>();

        let mut clean_file_name = String::new();
        for i in 0..(splitted_respect_file_name.len()) {
            clean_file_name = format!(
                "{}{}{}",
                clean_file_name,
                if i == 0 { "" } else { "_" },
                splitted_respect_file_name[i]
            );
        }

        let mappings = auto_fetch_all_mappings();
        let mut all_medias = match auto_fetch_all_medias(&mappings) {
            Ok(u) => u,
            _ => {
                return json!({"status": 500, "message": "Error: Failed fetching medias"});
            }
        };

        let end_block = EncryptionKey::generate_block(8);
        let media_id = EncryptionKey::generate_uuid(8);
        let final_name = format!("public/{}_{}.{}", clean_file_name, end_block, extension);

        match copy(path, final_name.clone()) {
            Err(_) => return json!({"status": 500, "message": "Error: File could not be saved"}),
            _ => {}
        };

        match Media::create(&mut all_medias, &media_id, &final_name) {
            Err(e) => return json!({"status": e.0, "message": e.1}),
            _ => {}
        }

        if let Err(e) = auto_save_all_medias(&mappings, &all_medias) {
            return json!({"status": 500, "message": e});
        }

        return json!({"status": 200, "message": "File uploaded successfully!", "path": final_name, "media_id": media_id});
    }

    return json!({"status": 500, "message": "Error: File could not be processed and/or saved"});
}
