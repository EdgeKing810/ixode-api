use crate::components::io::{fetch_file, save_file};
use magic_crypt::MagicCryptTrait;

#[derive(Default, Clone, Debug)]
pub struct EncryptionKey(pub String);

impl EncryptionKey {
    pub fn generate_block(length: usize) -> String {
        let charset = String::from("abcdefghijklmnopqrstuvwxyz0123456789");

        let mut uuid: String = String::new();

        for _ in 0..length {
            let index: f64 = fastrand::f64();
            let mut index_int: usize = (index * charset.len() as f64).round() as usize;

            if index_int >= charset.len() {
                index_int -= charset.len() - 1;
            }

            let mut chosen_char = charset.as_bytes()[index_int] as char;

            if fastrand::bool() && chosen_char.is_alphabetic() {
                chosen_char = chosen_char.to_uppercase().collect::<Vec<_>>()[0];
            }

            uuid.push(chosen_char);
        }

        uuid
    }

    pub fn generate_uuid(length: usize) -> String {
        let mut uuid: String = String::new();

        for _ in 0..5 {
            uuid = format!("{}{}{}", uuid, EncryptionKey::generate_block(length), "-");
        }

        let random_f: f64 = fastrand::f64();
        let index_int: u32 = (random_f * 9999 as f64).round() as u32;
        uuid = format!("{}{}", uuid, index_int.to_string());

        uuid
    }

    pub fn generate(length: usize) -> EncryptionKey {
        let key: String = EncryptionKey::generate_uuid(length);

        EncryptionKey(key)
    }

    pub fn encrypt(data: String, key: &str) -> String {
        let mc = new_magic_crypt!(key, 256);
        let ciphertext = mc.encrypt_str_to_base64(data);

        ciphertext
    }

    pub fn decrypt(data: String, key: &str) -> Result<EncryptionKey, String> {
        let mc = new_magic_crypt!(key, 256);
        let original_data = mc.decrypt_base64_to_string(&data);

        if let Err(e) = original_data {
            return Err(e.to_string());
        }

        Ok(EncryptionKey(original_data.unwrap()))
    }
}

pub fn fetch_encryption_key(path: String, password: &str) -> Result<String, String> {
    let encryption_key_raw = fetch_file(path.clone(), &String::from(password));

    if encryption_key_raw.split("\n").collect::<Vec<&str>>()[0] == ";|encrypted|;" {
        return Err(String::from("Error: Decryption failed"));
    }

    Ok(encryption_key_raw)
}

pub fn save_encryption_key(
    encryption_key: String,
    password: &str,
    path: &str,
) -> Result<(), String> {
    save_file(String::from(path), encryption_key, &String::from(password));
    println!("Encryption Key Saved!");

    Ok(())
}
