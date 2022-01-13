#![allow(dead_code)]
#[macro_use]
extern crate magic_crypt;
extern crate argon2;

mod init;
mod utils;

#[path = "components/components.rs"]
mod components;

use init::initialize;

#[path = "tests/tests.rs"]
mod tests;

fn main() {
    initialize();
}
