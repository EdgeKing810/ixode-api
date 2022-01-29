use regex::Regex;
use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::components::encryption::EncryptionKey;
use crate::components::user::{Role, User};
use crate::middlewares::token::{create_jwt, verify_jwt, Token};
use crate::utils::{
    auto_fetch_all_mappings, auto_fetch_all_users, auto_fetch_file, auto_save_all_users,
    get_config_value,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct NormalInput {
    uid: String,
}

#[post("/fetch", format = "json", data = "<data>")]
pub async fn fetch_all(data: Json<NormalInput>, token: Token) -> Value {
    let uid = &data.uid;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let mut clean_users = users.clone();
    User::remove_passwords(&mut clean_users);
    let amount = clean_users.len();

    return json!({"status": 200, "message": "Users fetched successfully!", "users": clean_users, "amount": amount});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserFetchInput {
    uid: String,
    target_uid: String,
}

#[post("/fetch/one", format = "json", data = "<data>")]
pub async fn fetch_one(data: Json<UserFetchInput>, token: Token) -> Value {
    let uid = &data.uid;
    let target_uid = &data.uid;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT && uid != target_uid {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let target_user = match User::get(&users, target_uid) {
        Ok(u) => u,
        Err(e) => {
            return json!({"status": 404, "message": format!("Error: User not found ({})", e)})
        }
    };

    let mut clean_users = Vec::<User>::new();
    clean_users.push(target_user);
    User::remove_passwords(&mut clean_users);

    return json!({"status": 200, "message": "Users fetched successfully!", "user": clean_users[0]});
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInput {
    auth_data: String,
    password: String,
}

#[post("/login", format = "json", data = "<data>")]
pub fn login(data: Json<LoginInput>) -> Value {
    let auth_data = &data.auth_data;
    let password = &data.password;

    let mappings = auto_fetch_all_mappings();
    let users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let mut is_username = true;

    let email_regex = Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .unwrap();
    if email_regex.is_match(auth_data) {
        is_username = false;
    }

    if (is_username && !User::exist_username(&users, &auth_data))
        || (!is_username && !User::exist_email(&users, &auth_data))
    {
        return json!({
            "status": 404,
            "message": format!("Error: Account with this {} not found",
            if is_username {"Username"} else {"Email Address"})
        });
    }

    let user = match if is_username {
        User::login_username(&users, &auth_data, &password)
    } else {
        User::login_email(&users, &auth_data, &password)
    } {
        Ok(user) => user,
        _ => return json!({"status": 401, "message": "Error: Incorrect Password"}),
    };

    let jwt = match create_jwt(&mappings, user.id.clone()) {
        Ok(token) => token,
        Err(e) => return json!({"status": 500, "message": e}),
    };

    json!({"status": 200, "message": "Works!", "user": user, "jwt": jwt})
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UIDInput {
    uid: String,
}

#[post("/verify", format = "json", data = "<data>")]
pub async fn verify(data: Json<UIDInput>, token: Token) -> Value {
    let uid = &data.uid;

    return match verify_jwt(uid.clone(), token.0).await {
        Ok(msg) => json!({"status": 200, "message": msg}),
        Err(info) => json!({"status": info.0, "message": info.1}),
    };
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterInput {
    uid: String,
    target_first_name: String,
    target_last_name: String,
    target_username: String,
    target_email: String,
    target_role_numeric: u32,
}

#[post("/register", format = "json", data = "<data>")]
pub async fn register(data: Json<RegisterInput>, token: Token) -> Value {
    let uid = &data.uid;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let target_first_name = &data.target_first_name;
    let target_last_name = &data.target_last_name;
    let target_username = &data.target_username;
    let target_email = &data.target_email;
    let target_role_numeric = &data.target_role_numeric;

    let password = EncryptionKey::generate_block(25);

    if User::exist_username(&mut users, target_username) {
        return json!({"status": 403, "message": "Error: Username is already in use"});
    }

    if User::exist_email(&mut users, target_email) {
        return json!({"status": 403, "message": "Error: Email Address is already in use"});
    }

    let new_user_uid = match User::create(
        &mut users,
        target_first_name,
        target_last_name,
        target_username,
        target_email,
        &password,
        target_role_numeric.clone(),
    ) {
        Ok(uid) => uid,
        Err(e) => return json!({"status": 400, "message": e}),
    };

    let smtp_username = get_config_value(&mappings, "SMTP_USERNAME", "");
    let smtp_password = get_config_value(&mappings, "SMTP_PASSWORD", "");
    let smtp_host = get_config_value(&mappings, "SMTP_HOST", "");
    let _smtp_port = get_config_value(&mappings, "SMTP_PORT", "");
    let project_name = get_config_value(&mappings, "PROJECT_NAME", "Kinesis API");
    let project_url = get_config_value(&mappings, "FRONT_URL", "https://front.kinesis.api");

    if smtp_username == "".to_string() || smtp_host == "".to_string() {
        return json!({"error": "500", "message": "Error: SMTP Settings not yet configured"});
    }

    let email_template = auto_fetch_file("templates/email/welcome.html", &mappings)
        .split("{name}")
        .fold(String::new(), |a, b| {
            a + &format!("{} {}", target_first_name, target_last_name) + b
        })
        .split("{site_url}")
        .fold(String::new(), |a, b| a + &project_url + b)
        .split("{site_name}")
        .fold(String::new(), |a, b| a + &project_name + b)
        .split("{username}")
        .fold(String::new(), |a, b| a + target_username + b)
        .split("{password}")
        .fold(String::new(), |a, b| a + &password + b);

    let email = Message::builder()
        .from(format!("Hello <{}>", smtp_username).parse().unwrap())
        .to(format!(
            "{} {} <{}>",
            target_first_name, target_last_name, target_email
        )
        .parse()
        .unwrap())
        .subject(format!("Welcome to {}", project_name))
        .body(email_template)
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    match auto_save_all_users(&mappings, &users) {
        Ok(_) => {
            return json!({"status": 200, "message": "User registered successfully!", "uid": new_user_uid})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Change {
    FIRSTNAME,
    LASTNAME,
    USERNAME,
    EMAIL,
    PASSWORD,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ChangeInput {
    uid: String,
    change: Change,
    data: String,
}

#[post("/update", format = "json", data = "<data>")]
pub async fn update(data: Json<ChangeInput>, token: Token) -> Value {
    let uid = &data.uid;
    let change = &data.change;
    let data = &data.data;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    match match change.clone() {
        Change::FIRSTNAME => User::update_first_name(&mut users, uid, data),
        Change::LASTNAME => User::update_last_name(&mut users, uid, data),
        Change::USERNAME => User::update_username(&mut users, uid, data),
        Change::EMAIL => User::update_email(&mut users, uid, data),
        Change::PASSWORD => User::update_password(&mut users, uid, data),
    } {
        Err(e) => {
            return json!({"status": 500, "message": e});
        }
        _ => {}
    }

    return match auto_save_all_users(&mappings, &users) {
        Ok(_) => json!({"status": 200, "message": "User updated successfully!"}),
        Err(e) => json!({"status": 500, "message": e}),
    };
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RoleInput {
    uid: String,
    target_uid: String,
    role: Role,
}

#[post("/update/role", format = "json", data = "<data>")]
pub async fn update_role(data: Json<RoleInput>, token: Token) -> Value {
    let uid = &data.uid;
    let target_uid = &data.target_uid;
    let role = &data.role;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let target_user = match User::get(&users, target_uid) {
        Ok(u) => u,
        Err(e) => {
            return json!({"status": 404, "message": format!("Error: User not found ({})", e)})
        }
    };
    if target_user.role == Role::ROOT {
        return json!({"status": 403, "message": "Error: Cannot change the ROLE of this User"});
    }

    let role_numeric: u32 = match role.clone() {
        Role::ROOT => 0,
        Role::ADMIN => 1,
        _ => 2,
    };

    match User::update_role(&mut users, target_uid, role_numeric) {
        Err(e) => return json!({"error": 500, "message": e}),
        _ => {}
    }

    return match auto_save_all_users(&mappings, &users) {
        Ok(_) => json!({"status": 200, "message": "User role updated successfully!"}),
        Err(e) => json!({"status": 500, "message": e}),
    };
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct DeleteInput {
    uid: String,
    target_uid: String,
}

#[post("/delete", format = "json", data = "<data>")]
pub async fn delete(data: Json<DeleteInput>, token: Token) -> Value {
    let uid = &data.uid;
    let target_uid = &data.target_uid;

    match verify_jwt(uid.clone(), token.0).await {
        Err(info) => return json!({"status": info.0, "message": info.1}),
        _ => {}
    };

    let mappings = auto_fetch_all_mappings();
    let mut users = match auto_fetch_all_users(&mappings) {
        Ok(u) => u,
        _ => {
            return json!({"status": 500, "message": "Error: Failed fetching users"});
        }
    };

    let current_user = User::get(&users, uid).unwrap();
    if current_user.role != Role::ROOT {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let target_user = match User::get(&users, target_uid) {
        Ok(u) => u,
        Err(e) => {
            return json!({"status": 404, "message": format!("Error: User not found ({})", e)})
        }
    };
    if target_user.role == Role::ROOT {
        return json!({"status": 403, "message": "Error: Cannot delete this User"});
    }

    match User::delete(&mut users, target_uid) {
        Err(e) => return json!({"error": 500, "message": e}),
        _ => {}
    }

    return match auto_save_all_users(&mappings, &users) {
        Ok(_) => json!({"status": 200, "message": "User deleted successfully!"}),
        Err(e) => json!({"status": 500, "message": e}),
    };
}
