#[cfg(test)]
use crate::{
    config::{fetch_all_configs, save_all_configs, Config},
    io::remove_file,
};

#[test]
fn test_configs() {
    let file_name: &str = "data/configs_test.txt";
    remove_file(file_name.to_string());

    let mut all_configs = fetch_all_configs(file_name.to_string(), &String::new());
    println!("{:#?}", all_configs);

    let test_config = Config::create(&mut all_configs, "TEST", "test");
    assert_eq!(test_config, Ok(()));

    let test_config2 = Config::create(&mut all_configs, "test?", "Test2");
    assert_eq!(
        test_config2,
        Err(String::from("Error: name contains an invalid character"))
    );

    let test_config2 = Config::create(&mut all_configs, "test", "Test2");
    assert_eq!(
        test_config2,
        Err(String::from(
            "Error: A config with that name already exists (TEST)"
        ))
    );

    let test_config2 = Config::create(&mut all_configs, "test2", "Test2|");
    assert_eq!(
        test_config2,
        Err(String::from("Error: value contains an invalid character"))
    );

    let test_config2 = Config::create(&mut all_configs, "test2", "Test2");
    assert_eq!(test_config2, Ok(()));

    let test2_id = "test2";

    let test_config2 = Config::update_value(&mut all_configs, test2_id, "TEST2VAL");
    assert_eq!(test_config2, Ok(()));

    save_all_configs(&all_configs, String::from(file_name), &String::new());
}
