use crate::components::{io::remove_file, mapping::Mapping};

pub fn reset_db(all_mappings: Vec<Mapping>, path: &str) {
    remove_file(path.to_string());
    for mapping in all_mappings.iter() {
        remove_file(mapping.get_file_name());
    }
}
