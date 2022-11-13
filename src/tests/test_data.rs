#[cfg(test)]
use crate::components::{
    data::{fetch_all_data, save_all_data, Data},
    io::remove_file,
};

#[test]
fn main() {
    let file_name: &str = "data/data_test.txt";
    remove_file(file_name.to_string());

    let mut all_data = fetch_all_data(file_name.to_string(), &String::new());
    println!("{:#?}", all_data);

    let test_data = Data::create(&mut all_data, "test", "test", "test", false);
    assert_eq!(test_data, Ok(()));

    let test_data2 = Data::create(&mut all_data, "test ", "test", "test", false);
    assert_eq!(
        test_data2,
        Err((400, String::from("Error: id contains an invalid character")))
    );

    let test_data2 = Data::create(&mut all_data, "test2", "test ***", "test", false);
    assert_eq!(
        test_data2,
        Err((
            400,
            String::from("Error: project_id contains an invalid character")
        ))
    );

    let test_data2 = Data::create(&mut all_data, "test2", "test2", "test2.", false);
    assert_eq!(
        test_data2,
        Err((
            400,
            String::from("Error: collection_id contains an invalid character")
        ))
    );

    let test_data2 = Data::create(&mut all_data, "test", "test2", "test2", false);
    assert_eq!(
        test_data2,
        Err((403, String::from("Error: id is already in use")))
    );

    let test_data2 = Data::create(&mut all_data, "test2", "test2", "test2", false);
    assert_eq!(test_data2, Ok(()));

    let test2_id = String::from("test2");

    let test_data3 = Data::update_project_id(&mut all_data, &test2_id, "test3");
    assert_eq!(test_data3, Ok(()));

    let test_data3 = Data::update_collection_id(&mut all_data, &test2_id, "test3");
    assert_eq!(test_data3, Ok(()));

    let test_data3 = Data::update_id(&mut all_data, &test2_id, "test3");
    assert_eq!(test_data3, Ok(()));

    save_all_data(&all_data, String::from(file_name), &String::new());
}
