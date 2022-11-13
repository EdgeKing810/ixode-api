use crate::components::{
    encryption::{fetch_encryption_key, save_encryption_key, EncryptionKey},
    mapping::{get_file_name, Mapping},
};

pub fn initialize_encryption_key(
    mappings: &Vec<Mapping>,
    password: &str,
) -> Result<String, String> {
    let encryption_key_path = get_file_name("encryption_key", mappings);
    let encryption_key: Result<String, String>;

    if let Err(e) = encryption_key_path {
        return Err(e);
    }

    encryption_key = fetch_encryption_key(encryption_key_path.clone().unwrap(), password);

    let mut should_continue = false;
    match encryption_key.clone() {
        Err(_) => {
            should_continue = true;
        }
        Ok(k) => {
            if k.len() < 1 {
                should_continue = true;
            }
        }
    }

    if should_continue {
        // Encryption key most likely doesn't exist yet
        let new_encryption_key = EncryptionKey::generate(32);
        let saved_encryption_key = save_encryption_key(
            new_encryption_key.0.clone(),
            password,
            &*encryption_key_path.unwrap(),
        );

        if let Err(f) = saved_encryption_key {
            return Err(String::from(f));
        }

        println!("Encryption Key Saved!");

        return Ok(new_encryption_key.0);
    }

    Ok(encryption_key.unwrap())
}
