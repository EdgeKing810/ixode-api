#[cfg(test)]
use crate::components::{
    encryption::{fetch_encryption_key, save_encryption_key, EncryptionKey},
    io::remove_file,
};

#[test]
fn test_encryption() {
    let file_name: &str = "data/encryption_key_test.txt";
    remove_file(file_name.to_string());

    let password: &str = "Test123*";
    let length: usize = 30;

    let encryption_key = fetch_encryption_key(file_name.to_string(), password);
    println!("{:#?}", encryption_key);

    let generated_encryption_key = EncryptionKey::generate(length);
    let encrypted_generated_encryption_key =
        EncryptionKey::encrypt(generated_encryption_key.0.clone(), password);
    let decrypted_generated_encryption_key =
        EncryptionKey::decrypt(encrypted_generated_encryption_key, password);

    assert_eq!(
        decrypted_generated_encryption_key.unwrap().0,
        generated_encryption_key.0
    );

    let saved_encryption_key = save_encryption_key(generated_encryption_key.0, password, file_name);
    if let Err(e) = saved_encryption_key {
        println!("Error: {}", e);
    }
}
