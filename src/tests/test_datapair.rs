#[cfg(test)]
use crate::components::{data::Data, datapair::DataPair};

#[test]
fn main() {
    let mut all_pairs = Vec::<DataPair>::new();

    let test_pair = DataPair::create(&mut all_pairs, "test", "_", "", "test", "string");
    assert_eq!(test_pair, Ok(()));

    let test_pair2 = DataPair::create(&mut all_pairs, "test ", "_", "", "test2", "test2");
    assert_eq!(
        test_pair2,
        Err((400, String::from("Error: id contains an invalid character")))
    );

    let test_pair2 = DataPair::create(
        &mut all_pairs,
        "test2",
        "test2.",
        "test2",
        "test2",
        "test2.",
    );
    assert_eq!(
        test_pair2,
        Err((
            400,
            String::from("Error: structure_id contains an invalid character")
        ))
    );

    let test_pair2 = DataPair::create(
        &mut all_pairs,
        "test2",
        "test2",
        "test2.",
        "test2",
        "test2.",
    );
    assert_eq!(
        test_pair2,
        Err((
            400,
            String::from("Error: custom_structure_id contains an invalid character")
        ))
    );

    let test_pair2 = DataPair::create(&mut all_pairs, "test2", "_", "", "test2", "test2.");
    assert_eq!(
        test_pair2,
        Err((
            400,
            String::from("Error: dtype contains an invalid character")
        ))
    );

    let test_pair2 = DataPair::create(&mut all_pairs, "test", "_", "", "test2", "test2");
    assert_eq!(
        test_pair2,
        Err((403, String::from("Error: id is already in use")))
    );

    let test_pair2 = DataPair::create(&mut all_pairs, "test2", "_", "test2", "test2", "test2");
    assert_eq!(test_pair2, Ok(()));

    let pair2_id = String::from("test2");

    let test_pair3 = DataPair::update_value(&mut all_pairs, &pair2_id, "test3");
    assert_eq!(test_pair3, Ok(()));

    let test_pair3 = DataPair::update_custom_structure_id(&mut all_pairs, &pair2_id, "test3");
    assert_eq!(test_pair3, Ok(()));

    let test_pair3 = DataPair::update_dtype(&mut all_pairs, &pair2_id, "test3");
    assert_eq!(test_pair3, Ok(()));

    let test_pair3 = DataPair::update_id(&mut all_pairs, &pair2_id, "test3");
    assert_eq!(test_pair3, Ok(()));

    let mut all_data = Vec::<Data>::new();
    let test_pair4 = Data::add_pair(&mut all_data, &String::from("test"), all_pairs[0].clone());
    assert_eq!(
        test_pair4,
        Err((404, String::from("Error: Data not found")))
    );

    let create_data = Data::create(&mut all_data, "test", "test", "test", false);
    assert_eq!(create_data, Ok(()));

    let test_pair4 = Data::add_pair(&mut all_data, &String::from("test"), all_pairs[0].clone());
    assert_eq!(test_pair4, Ok(()));

    let test_pair4 = Data::update_pair(
        &mut all_data,
        &String::from("test"),
        &String::from("test"),
        all_pairs[1].clone(),
    );
    assert_eq!(test_pair4, Ok(()));

    let test_pair4 =
        Data::remove_pair(&mut all_data, &String::from("test"), &String::from("test4"));
    assert_eq!(
        test_pair4,
        Err((404, String::from("Error: DataPair not found")))
    );

    let test_pair4 =
        Data::remove_pair(&mut all_data, &String::from("test"), &String::from("test3"));
    assert_eq!(test_pair4, Ok(()));
}
