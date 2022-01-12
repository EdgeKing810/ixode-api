#![allow(unused_assignments)]
#[cfg(test)]
#[path = "test_mappings.rs"]
mod test_mappings;

#[path = "test_users.rs"]
mod test_users;

#[path = "test_projects.rs"]
mod test_projects;

#[path = "test_configs.rs"]
mod test_configs;

#[path = "test_encryption.rs"]
mod test_encryption;

#[path = "test_collections.rs"]
mod test_collections;
