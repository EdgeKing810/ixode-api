use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FailObj {
    pub status: u32,
    pub message: String,
}

impl FailObj {
    pub fn create(status: u32, message: &str) -> Result<FailObj, (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let mut fail_obj = FailObj {
            status: status,
            message: "".to_string(),
        };

        let message_update = Self::update_message(&mut fail_obj, message);
        if let Err(e) = message_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        if has_error {
            return Err(latest_error);
        }

        Ok(fail_obj)
    }

    pub fn update_message(fail_obj: &mut FailObj, message: &str) -> Result<(), (usize, String)> {
        if message.trim().len() > 200 {
            return Err((
                400,
                String::from("Error: message contains too many characters"),
            ));
        }

        if !message.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == '_'
                || c == '-'
                || c == ':'
                || c == ';'
                || c == ' '
                || c == '.'
        }) {
            return Err((
                400,
                String::from("Error: message contains an invalid character"),
            ));
        }

        fail_obj.message = message.to_string();

        Ok(())
    }

    pub fn to_string(fail_obj: FailObj) -> String {
        format!("[{},{}]", fail_obj.status, fail_obj.message)
    }

    pub fn from_string(fail_obj_str: &str) -> Result<FailObj, (usize, String)> {
        let mut current_fail_obj = fail_obj_str.split("[").collect::<Vec<&str>>();
        if current_fail_obj.len() <= 1 {
            return Err((500, String::from("Error: Invalid fail_obj string / 1")));
        }

        current_fail_obj = current_fail_obj[1].split("]").collect::<Vec<&str>>();
        if current_fail_obj.len() <= 1 {
            return Err((500, String::from("Error: Invalid fail_obj string / 2")));
        }

        current_fail_obj = current_fail_obj[0].split(",").collect::<Vec<&str>>();
        if current_fail_obj.len() < 2 {
            return Err((500, String::from("Error: Invalid fail_obj string / 3")));
        }

        let status = match current_fail_obj[0].trim().parse::<u32>() {
            Ok(sts) => sts,
            Err(e) => return Err((500, format!("Error: Invalid fail_obj string / 4: {}", e))),
        };

        return FailObj::create(status, current_fail_obj[1]);
    }
}
