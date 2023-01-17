use crate::{
    components::io::{fetch_file, save_file},
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

use rocket::serde::{Deserialize, Serialize};
use uuid::Uuid;

use argon2::{self, Config};
use regex::Regex;

use super::constraint_property::ConstraintProperty;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Role {
    ROOT,
    ADMIN,
    AUTHOR,
    VIEWER,
}

impl Default for Role {
    fn default() -> Self {
        Role::VIEWER
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: String,
    first_name: String,
    last_name: String,
    pub username: String,
    email: String,
    password: String,
    pub role: Role,
}

impl User {
    fn create_no_check(
        id: &str,
        first_name: &str,
        last_name: &str,
        username: &str,
        email: &str,
        password: &str,
        role: Role,
    ) -> User {
        User {
            id: String::from(id),
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            username: String::from(username),
            email: String::from(email),
            password: String::from(password),
            role,
        }
    }

    pub fn get(all_users: &Vec<User>, uid: &str) -> Result<User, (usize, String)> {
        for user in all_users.iter() {
            if user.id.to_lowercase() == uid.to_lowercase() {
                return Ok(user.clone());
            }
        }

        Err((404, String::from("Error: User not found")))
    }

    pub fn exist(all_users: &Vec<User>, id: &str) -> bool {
        let mut found = false;
        for user in all_users.iter() {
            if user.id == id {
                found = true;
                break;
            }
        }

        found
    }

    pub fn exist_username(all_users: &Vec<User>, username: &str) -> bool {
        let mut found = false;
        for user in all_users.iter() {
            if user.username.to_lowercase() == username.to_lowercase() {
                found = true;
                break;
            }
        }

        found
    }

    pub fn exist_email(all_users: &Vec<User>, email: &str) -> bool {
        let mut found = false;
        for user in all_users.iter() {
            if user.email.to_lowercase() == email.to_lowercase() {
                found = true;
                break;
            }
        }

        found
    }

    pub fn register(
        all_users: &mut Vec<User>,
        first_name: &str,
        last_name: &str,
        username: &str,
        email: &str,
        password: &str,
        role_numeric: u32,
    ) -> Result<String, (usize, String)> {
        return User::create(
            all_users,
            first_name,
            last_name,
            username,
            email,
            password,
            role_numeric,
        );
    }

    pub fn create(
        all_users: &mut Vec<User>,
        first_name: &str,
        last_name: &str,
        username: &str,
        email: &str,
        password: &str,
        role_numeric: u32,
    ) -> Result<String, (usize, String)> {
        let id = Uuid::new_v4();
        let uid = id.to_string();

        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let salt = Uuid::new_v4();
        let config = Config::default();

        let new_user = User {
            id: uid.clone(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            username: "".to_string(),
            email: "".to_string(),
            password: argon2::hash_encoded("tmp".as_bytes(), salt.as_bytes(), &config)
                .unwrap()
                .to_string(),
            role: Role::default(),
        };
        all_users.push(new_user);

        let name_update = Self::update_name(all_users, &uid, first_name, last_name);
        if let Err(e) = name_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        if !has_error {
            let username_update = Self::update_username(all_users, &uid, username);
            if let Err(e) = username_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let email_update = Self::update_email(all_users, &uid, email);
            if let Err(e) = email_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let password_update = Self::update_password(all_users, &uid, password);
            if let Err(e) = password_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if !has_error {
            let role_update = Self::update_role(all_users, &uid, role_numeric);
            if let Err(e) = role_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            let delete_user = Self::delete(all_users, &uid);
            if let Err(e) = delete_user {
                println!("{}", e.1);
            }

            return Err(latest_error);
        }

        Ok(uid)
    }

    pub fn login_username(
        all_users: &Vec<User>,
        auth: &str,
        password: &str,
    ) -> Result<User, (usize, String)> {
        let mut found_user: Option<User> = None;

        for user in all_users.iter() {
            if user.username == auth.to_string() {
                found_user = Some(user.clone());
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        let correct_password = match argon2::verify_encoded(
            &found_user.clone().unwrap().password,
            password.as_bytes(),
        ) {
            Ok(b) => b,
            Err(e) => {
                return Err((400, e.to_string()));
            }
        };

        if !correct_password {
            return Err((401, String::from("Error: Password mismatch")));
        }

        Ok(found_user.unwrap())
    }

    pub fn login_email(
        all_users: &Vec<User>,
        auth: &str,
        password: &str,
    ) -> Result<User, (usize, String)> {
        let mut found_user: Option<User> = None;

        for user in all_users.iter() {
            if user.email == auth.to_string() {
                found_user = Some(user.clone());
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        let correct_password =
            argon2::verify_encoded(&found_user.clone().unwrap().password, password.as_bytes());

        if !correct_password.is_ok() {
            return Err((401, String::from("Error: Password mismatch")));
        }

        Ok(found_user.unwrap())
    }

    pub fn update_first_name(
        all_users: &mut Vec<User>,
        id: &String,
        first_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_user: Option<User> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value = match ConstraintProperty::validate(
            &all_constraints,
            "user",
            "first_name",
            first_name,
        ) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };

        for user in all_users.iter_mut() {
            if user.id == id.to_string() {
                found_user = Some(user.clone());
                user.first_name = final_value;
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        Ok(())
    }

    pub fn update_last_name(
        all_users: &mut Vec<User>,
        id: &String,
        last_name: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_user: Option<User> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "user", "last_name", last_name) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for user in all_users.iter_mut() {
            if user.id == id.to_string() {
                found_user = Some(user.clone());
                user.last_name = final_value;
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        Ok(())
    }

    pub fn update_name(
        all_users: &mut Vec<User>,
        id: &String,
        first_name: &str,
        last_name: &str,
    ) -> Result<(), (usize, String)> {
        return match User::update_first_name(all_users, id, first_name) {
            Ok(()) => match User::update_last_name(all_users, id, last_name) {
                Ok(()) => Ok(()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        };
    }

    pub fn update_username(
        all_users: &mut Vec<User>,
        id: &String,
        username: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_user: Option<User> = None;

        for user in all_users.iter() {
            if user.username.to_lowercase() == username.to_lowercase().trim() && user.id != *id {
                return Err((403, String::from("Error: username already taken")));
            }
        }

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "user", "username", username) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for user in all_users.iter_mut() {
            if user.id == id.to_string() {
                found_user = Some(user.clone());
                user.username = final_value;
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        Ok(())
    }

    pub fn update_email(
        all_users: &mut Vec<User>,
        id: &String,
        email: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_user: Option<User> = None;

        for user in all_users.iter() {
            if user.email.to_lowercase() == email.to_lowercase().trim() && user.id != *id {
                return Err((403, String::from("Error: email already taken")));
            }
        }

        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        if !email_regex.is_match(email) {
            return Err((400, String::from("Error: Invalid email address")));
        }

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "user", "email", email) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        for user in all_users.iter_mut() {
            if user.id == id.to_string() {
                found_user = Some(user.clone());
                user.email = final_value;
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        Ok(())
    }

    pub fn update_password(
        all_users: &mut Vec<User>,
        id: &String,
        password: &str,
    ) -> Result<(), (usize, String)> {
        let mut found_user: Option<User> = None;

        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "user", "password", password) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        if !final_value
            .trim()
            .chars()
            .any(|c| c.is_alphabetic() && c.is_uppercase())
        {
            return Err((
                400,
                String::from(
                    "Error: password should contain at least 1 uppercase alphabetic character",
                ),
            ));
        } else if !final_value
            .trim()
            .chars()
            .any(|c| c.is_alphabetic() && c.is_lowercase())
        {
            return Err((
                400,
                String::from(
                    "Error: password should contain at least 1 lowercase alphabetic character",
                ),
            ));
        } else if !final_value.trim().chars().any(|c| c.is_numeric()) {
            return Err((
                400,
                String::from("Error: password should contain at least 1 number"),
            ));
        }

        for user in all_users.iter_mut() {
            if user.id == id.to_string() {
                let salt = Uuid::new_v4();
                let config = Config::default();

                found_user = Some(user.clone());
                user.password =
                    argon2::hash_encoded(final_value.as_bytes(), salt.as_bytes(), &config)
                        .unwrap()
                        .to_string();
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        Ok(())
    }

    pub fn update_role(
        all_users: &mut Vec<User>,
        id: &String,
        role_numeric: u32,
    ) -> Result<(), (usize, String)> {
        let mut found_user: Option<User> = None;

        let role = match role_numeric {
            0 => Role::ROOT,
            1 => Role::ADMIN,
            2 => Role::AUTHOR,
            _ => Role::VIEWER,
        };

        for user in all_users.iter_mut() {
            if user.id == id.to_string() {
                found_user = Some(user.clone());
                user.role = role;
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        Ok(())
    }

    pub fn delete(all_users: &mut Vec<User>, id: &String) -> Result<(), (usize, String)> {
        let mut found_user: Option<User> = None;

        for user in all_users.iter_mut() {
            if user.id == id.to_string() {
                found_user = Some(user.clone());
                break;
            }
        }

        if let None = found_user {
            return Err((404, String::from("Error: User not found")));
        }

        let updated_users: Vec<User> = all_users
            .iter_mut()
            .filter(|user| user.id != *id)
            .map(|user| User {
                id: user.id.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                username: user.username.clone(),
                email: user.email.clone(),
                password: user.password.clone(),
                role: user.role.clone(),
            })
            .collect::<Vec<User>>();

        *all_users = updated_users;

        Ok(())
    }

    pub fn remove_passwords(all_users: &mut Vec<User>) {
        let updated_users: Vec<User> = all_users
            .iter_mut()
            .map(|user| User {
                id: user.id.clone(),
                first_name: user.first_name.clone(),
                last_name: user.last_name.clone(),
                username: user.username.clone(),
                email: user.email.clone(),
                password: String::from(""),
                role: user.role.clone(),
            })
            .collect::<Vec<User>>();

        *all_users = updated_users;
    }

    pub fn obtain_properties() -> String {
        String::from("id;first_name;last_name;username;email;password;role")
    }

    pub fn to_string(user: User) -> String {
        let number_role: u32 = match user.role {
            Role::ROOT => 0,
            Role::ADMIN => 1,
            Role::AUTHOR => 2,
            _ => 3,
        };

        format!(
            "{};{};{};{};{};{};{}",
            user.id,
            user.first_name,
            user.last_name,
            user.username,
            user.email,
            user.password,
            number_role
        )
    }

    pub fn from_string(user_str: &str) -> User {
        let current_user = user_str.split(";").collect::<Vec<&str>>();

        let parsed_role_raw = current_user[6].parse::<u32>();
        if let Err(e) = parsed_role_raw.clone() {
            println!("Error when parsing role: {}", e);
        }

        let mut parsed_role: u32 = 2;
        if let Ok(val) = parsed_role_raw {
            parsed_role = val;
        }

        let role = match parsed_role {
            0 => Role::ROOT,
            1 => Role::ADMIN,
            2 => Role::AUTHOR,
            _ => Role::VIEWER,
        };

        User::create_no_check(
            current_user[0],
            current_user[1],
            current_user[2],
            current_user[3],
            current_user[4],
            current_user[5],
            role,
        )
    }
}

pub fn stringify_users(users: &Vec<User>) -> String {
    let mut stringified_users = String::new();

    for user in users {
        stringified_users = format!(
            "{}{}{}",
            stringified_users,
            if stringified_users.chars().count() > 1 {
                "\n"
            } else {
                ""
            },
            User::to_string(user.clone())
        );
    }

    stringified_users
}

pub fn unwrap_users(all_users_raw: String) -> Vec<User> {
    let individual_users = all_users_raw
        .split("\n")
        .filter(|line| line.chars().count() >= 3);

    let mut final_users: Vec<User> = Vec::<User>::new();

    for user in individual_users {
        let tmp_user = User::from_string(user);
        final_users.push(tmp_user);
    }

    final_users
}

pub fn fetch_all_users(path: String, encryption_key: &String) -> Vec<User> {
    let all_users_raw = fetch_file(path.clone(), encryption_key);
    let final_users = unwrap_users(all_users_raw);
    final_users
}

pub fn save_all_users(users: &Vec<User>, path: String, encryption_key: &String) {
    let stringified_users = stringify_users(users);
    save_file(path, stringified_users, encryption_key);
    println!("Users saved!");
}
