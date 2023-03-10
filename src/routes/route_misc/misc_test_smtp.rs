use rocket::post;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::{Deserialize, Serialize};

use lettre::message::MultiPart;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::components::user::{Role, User};
use crate::middlewares::token::{verify_jwt, Token};
use crate::utils::{
    config::get_config_value, io::auto_fetch_file, mapping::auto_fetch_all_mappings,
    user::auto_fetch_all_users,
};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TestSMTPInput {
    uid: String,
    username: String,
    password: String,
    host: String,
    port: String,
}

#[post("/test/smtp", format = "json", data = "<data>")]
pub async fn main(data: Json<TestSMTPInput>, token: Token) -> Value {
    let uid = &data.uid;
    let username = &data.username;
    let password = &data.password;
    let host = &data.host;
    let _port = &data.port;

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
    if current_user.role != Role::ROOT && current_user.role != Role::ADMIN {
        return json!({"status": 403, "message": "Error: Not enough privileges to carry out this operation"});
    }

    let email_template = auto_fetch_file("templates/email/test.html", &mappings);
    let fake_recipient = get_config_value(&mappings, "SMTP_FAKE_RECIPIENT", "hello@kinesis.world");

    if username.len() == 0 || password.len() == 0 || host.len() == 0 {
        return json!({"status": 500, "message": "Error: Missing SMTP credentials"});
    }

    let email = Message::builder()
        .from(match format!("Hello <{}>", username).parse() {
            Ok(e) => e,
            Err(e) => return json!({"status": 500, "message": format!("Error: {}", e.to_string())}),
        })
        .to(format!("Test Recipient <{}>", fake_recipient)
            .parse()
            .unwrap())
        .subject("IGNORE THIS. Testing SMTP Credentials")
        .multipart(MultiPart::alternative_plain_html(
            String::from(""),
            email_template,
        ))
        .unwrap();

    let creds = Credentials::new(username.clone(), password.clone());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(&host)
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email successfully sent!"),
        Err(e) => {
            return json!({"status": 500, "message": format!("Error: Email could not be sent [{}]", e)})
        }
    }

    return json!({"status": 200, "message": "Valid SMTP Credentials!"});
}
