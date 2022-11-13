#[cfg(test)]
#[test]
fn main() {
    use crate::components::constraint_property::ConstraintProperty;
    use crate::components::{
        constraint::{fetch_all_constraints, save_all_constraints, Constraint},
        io::remove_file,
    };

    let file_name: &str = "data/constraints_test.txt";
    remove_file(file_name.to_string());

    let mut all_constraints = fetch_all_constraints(file_name.to_string(), &String::new()).unwrap();
    println!("{:#?}", all_constraints);

    let test_constraint = Constraint::create(&mut all_constraints, "test");
    assert_eq!(test_constraint, Ok(()));

    let test_constraint2 = Constraint::create(&mut all_constraints, "test?");
    assert_eq!(
        test_constraint2,
        Err((
            400,
            String::from("Error: component_name contains an invalid character")
        ))
    );

    let test_constraint2 = Constraint::create(&mut all_constraints, "test");
    assert_eq!(
        test_constraint2,
        Err((
            403,
            String::from("Error: A constraint with that component_name already exists (test)")
        ))
    );

    let test_constraint2 = Constraint::create(&mut all_constraints, "test2");
    assert_eq!(test_constraint2, Ok(()));

    let test2_id = "test2";

    let test_constraint2 = Constraint::update_name(&mut all_constraints, test2_id, "test3");
    assert_eq!(test_constraint2, Ok(()));

    let mut all_properties = Vec::<ConstraintProperty>::new();
    let test_property = ConstraintProperty::create(
        &mut all_properties,
        "test_prop",
        true,
        true,
        0,
        0,
        vec![],
        vec![],
    );
    assert_eq!(test_property, Ok(()));

    let test_property2 = ConstraintProperty::create(
        &mut all_properties,
        "test_prop!",
        true,
        true,
        0,
        0,
        vec![],
        vec![],
    );
    assert_eq!(
        test_property2,
        Err((
            400,
            String::from("Error: property_name contains an invalid character")
        ))
    );

    let test_property2 = ConstraintProperty::create(
        &mut all_properties,
        "test_prop",
        true,
        true,
        0,
        0,
        vec![],
        vec![],
    );
    assert_eq!(
        test_property2,
        Err((
            403,
            String::from(
                "Error: A constraint property with that property_name already exists (test_prop)"
            )
        ))
    );

    let test_property2 = ConstraintProperty::create(
        &mut all_properties,
        "test_prop2",
        true,
        true,
        0,
        0,
        vec![],
        vec![],
    );
    assert_eq!(test_property2, Ok(()));

    let test_property2 =
        ConstraintProperty::update_property_name(&mut all_properties, "test_prop2", "test_prop3!");
    assert_eq!(
        test_property2,
        Err((
            400,
            String::from("Error: property_name contains an invalid character")
        ))
    );

    let test_property2 =
        ConstraintProperty::add_not_allowed(&mut all_properties, "test_prop2", '\n');
    assert_eq!(
        test_property2,
        Err((400, String::from("Error: Invalid value for character")))
    );

    let test_property2 =
        ConstraintProperty::add_not_allowed(&mut all_properties, "test_prop2", 'a');
    assert_eq!(test_property2, Ok(()));

    let test_property2 =
        ConstraintProperty::add_additional_allowed(&mut all_properties, "test_prop2", '\n');
    assert_eq!(
        test_property2,
        Err((400, String::from("Error: Invalid value for character")))
    );

    let test_property2 =
        ConstraintProperty::add_additional_allowed(&mut all_properties, "test_prop2", 'a');
    assert_eq!(test_property2, Ok(()));

    let test_constraint2 =
        Constraint::set_properties(&mut all_constraints, "test3", all_properties);
    assert_eq!(test_constraint2, Ok(()));

    save_all_constraints(&all_constraints, String::from(file_name), &String::new());
}
