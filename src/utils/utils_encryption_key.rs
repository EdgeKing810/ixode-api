use crate::{components::mapping::Mapping, init::encryption_key::initialize_encryption_key};

pub fn get_encryption_key(all_mappings: &Vec<Mapping>, tmp_password: &str) -> String {
    let init_encryption = initialize_encryption_key(&all_mappings, tmp_password);

    if let Err(e) = init_encryption {
        println!("Error: {}", e);
        return String::new();
    }

    init_encryption.unwrap()
}
