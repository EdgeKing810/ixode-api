#[cfg(test)]
use crate::components::{
    io::remove_file,
    user::{fetch_all_users, save_all_users, User},
};

#[test]
fn test_users() {
    let file_name: &str = "data/users_test.txt";
    remove_file(file_name.to_string());

    let mut all_users = fetch_all_users(file_name.to_string(), &String::new());
    println!("{:#?}", all_users);

    let test_user = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test",
        "test@test.com",
        "Test123*",
        0,
    );
    assert_eq!(test_user.is_ok(), true);

    let test_user2 = User::create(
        &mut all_users,
        "Test2",
        "Tester",
        "test2",
        "test@test2.com",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: first_name contains an invalid character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester2",
        "test2",
        "test@test2.com",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: last_name contains an invalid character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test",
        "test@test2.com",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from("Error: username already taken"))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test.com",
        "Test123*",
        0,
    );
    assert_eq!(test_user2, Err(String::from("Error: email already taken")));

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@@test2.teeeeeeeeest",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from("Error: Invalid email address"))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2..teeeeeeeeest",
        "Test123*",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from("Error: Invalid email address"))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "Test",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should be longer than 7 characters"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "testtest",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should contain at least 1 uppercase alphabetic character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "TESTTEST",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should contain at least 1 lowercase alphabetic character"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "testTEST",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password should contain at least 1 number"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "test2",
        "test@test2.com",
        "Test123;",
        0,
    );
    assert_eq!(
        test_user2,
        Err(String::from(
            "Error: password contains a forbidden character (;)"
        ))
    );

    let test_user2 = User::create(
        &mut all_users,
        "Test",
        "Tester",
        "te_st",
        "test@test2.com",
        "Test123*&^()[]{}*-_",
        0,
    );
    assert_eq!(test_user2.is_ok(), true);

    let incorrect_login_test_user2 = User::login_username(&all_users, "te_st", "Test123*");
    assert_eq!(
        incorrect_login_test_user2,
        Err(String::from("Error: Password mismatch"))
    );

    let login_test_user2 = User::login_username(&all_users, "te_st", "Test123*&^()[]{}*-_");

    if let Ok(successful_login) = login_test_user2 {
        let test_user2 = User::update_name(&mut all_users, &successful_login.id, "Test", "Tester");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_username(&mut all_users, &successful_login.id, "test2");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_email(&mut all_users, &successful_login.id, "test2@test.com");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_password(&mut all_users, &successful_login.id, "Test123*");
        assert_eq!(test_user2, Ok(()));

        let test_user2 = User::update_role(&mut all_users, &successful_login.id, 2);
        assert_eq!(test_user2, Ok(()));
    };

    save_all_users(&all_users, String::from(file_name), &String::new());
}
