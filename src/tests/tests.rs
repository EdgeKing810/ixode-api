#![allow(unused_assignments)]
#[cfg(test)]
#[path = "test_mapping.rs"]
mod test_mapping;

#[path = "test_user.rs"]
mod test_user;

#[path = "test_project.rs"]
mod test_project;

#[path = "test_config.rs"]
mod test_config;

#[path = "test_encryption.rs"]
mod test_encryption;

#[path = "test_collection.rs"]
mod test_collection;

#[path = "test_media.rs"]
mod test_media;

#[path = "test_data.rs"]
mod test_data;

#[path = "test_event.rs"]
mod test_event;

#[path = "test_datapair.rs"]
mod test_datapair;

#[path = "test_constraint.rs"]
mod test_constraint;

#[path = "../components/routing/tests/tests.rs"]
pub mod test_routing;
