use crate::{
    components::{
        mapping::{get_file_name, Mapping},
        user::{fetch_all_users, save_all_users, User},
    },
    utils::encryption_key::get_encryption_key,
};

pub fn initialize_users(mappings: &Vec<Mapping>) -> Vec<User> {
    let all_users_path = get_file_name("users", mappings);
    let mut all_users = Vec::<User>::new();

    if let Err(e) = all_users_path {
        println!("{}", e);
        return all_users;
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    all_users = fetch_all_users(
        all_users_path.clone().unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    if !User::exist_username(&all_users, "EdgeKing810") {
        let create_user = User::create(
            &mut all_users,
            "Kishan",
            "Takoordyal",
            "EdgeKing810",
            "kishan@konnect.dev",
            "Test123*",
            0,
        );
        if let Err(e) = create_user {
            println!("{}", e.1);
        }
    }

    let pass = match std::env::var("TMP_PASSWORD") {
        Ok(p) => p,
        _ => "Test123*".to_string(),
    };

    save_all_users(
        &all_users,
        all_users_path.unwrap(),
        &get_encryption_key(&mappings, &pass),
    );

    all_users
}
