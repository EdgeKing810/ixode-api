use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::components::encryption::EncryptionKey;
use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::config::get_config_value;
use crate::utils::event::auto_create_event;
use crate::utils::io::auto_fetch_file;
use crate::utils::mapping::auto_fetch_all_mappings;
use crate::utils::user::{auto_fetch_all_users, auto_save_all_users};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCreateInput {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    role_numeric: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RegisterInput {
    uid: String,
    user: UserCreateInput,
}

#[post("/register", format = "json", data = "<data>")]
pub async fn main(data: Json<RegisterInput>, token: Token) -> Value {
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

    let first_name = &data.user.first_name;
    let last_name = &data.user.last_name;
    let username = &data.user.username;
    let email = &data.user.email;
    let role_numeric = &data.user.role_numeric;

    let password = EncryptionKey::generate_block(25);

    if User::exist_username(&mut users, username) {
        return json!({"status": 403, "message": "Error: Username is already in use"});
    }

    if User::exist_email(&mut users, email) {
        return json!({"status": 403, "message": "Error: Email Address is already in use"});
    }

    let new_user_uid = match User::create(
        &mut users,
        first_name,
        last_name,
        username,
        email,
        &password,
        role_numeric.clone(),
    ) {
        Ok(uid) => uid,
        Err(e) => return json!({"status": e.0, "message": e.1}),
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
        .collect::<Vec<&str>>()
        .join(&format!("{} {}", first_name, last_name))
        .split("{site_url}")
        .collect::<Vec<&str>>()
        .join(&format!("{}", project_url))
        .split("{site_name}")
        .collect::<Vec<&str>>()
        .join(&format!("{}", project_name))
        .split("{username}")
        .collect::<Vec<&str>>()
        .join(&format!("{}", username))
        .split("{password}")
        .collect::<Vec<&str>>()
        .join(&format!("{}", password));

    let email_to_be_sent = Message::builder()
        .from(format!("Hello <{}>", smtp_username).parse().unwrap())
        .to(format!("{} {} <{}>", first_name, last_name, email)
            .parse()
            .unwrap())
        .subject(format!("Welcome to {}", project_name))
        .multipart(MultiPart::alternative_plain_html(
            String::from(""),
            email_template,
        ))
        .unwrap();

    let creds = Credentials::new(smtp_username, smtp_password);

    let mailer = SmtpTransport::relay(&smtp_host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email_to_be_sent) {
        Ok(_) => println!("Email successfully sent!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }

    if let Err(e) = auto_create_event(
        &mappings,
        "user_register",
        format!(
            "A new user with the email address <{}> was registered by usr[{}]",
            email, uid
        ),
        format!("/users"),
    ) {
        return json!({"status": e.0, "message": e.1});
    }

    match auto_save_all_users(&mappings, &users) {
        Ok(_) => {
            return json!({"status": 200, "message": "User successfully registered!", "uid": new_user_uid})
        }
        Err(e) => {
            json!({"status": 500, "message": e})
        }
    }
}
