use rocket::serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthJWT {
    pub active: bool,
    pub field: String,
    pub ref_col: String,
}

impl AuthJWT {
    pub fn create(active: bool, field: &str, ref_col: &str) -> Result<AuthJWT, (usize, String)> {
        let mut has_error: bool = false;
        let mut latest_error: (usize, String) = (500, String::new());

        let mut auth_jwt_obj = AuthJWT {
            active: active,
            field: "".to_string(),
            ref_col: "".to_string(),
        };

        let field_update = Self::update_field(&mut auth_jwt_obj, field);
        if let Err(e) = field_update {
            has_error = true;
            println!("{}", e.1);
            latest_error = e;
        }

        if !has_error {
            let ref_col_update = Self::update_ref_col(&mut auth_jwt_obj, ref_col);
            if let Err(e) = ref_col_update {
                has_error = true;
                println!("{}", e.1);
                latest_error = e;
            }
        }

        if has_error {
            return Err(latest_error);
        }

        Ok(auth_jwt_obj)
    }

    pub fn update_field(auth_obj: &mut AuthJWT, field: &str) -> Result<(), (usize, String)> {
        if field.trim().len() > 100 {
            return Err((
                400,
                String::from("Error: field contains too many characters"),
            ));
        }

        if !field
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: field contains an invalid character"),
            ));
        }

        auth_obj.field = field.to_string();

        Ok(())
    }

    pub fn update_ref_col(auth_obj: &mut AuthJWT, ref_col: &str) -> Result<(), (usize, String)> {
        if ref_col.trim().len() > 100 {
            return Err((
                400,
                String::from("Error: ref_col contains too many characters"),
            ));
        }

        if !ref_col
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
        {
            return Err((
                400,
                String::from("Error: ref_col contains an invalid character"),
            ));
        }

        auth_obj.ref_col = ref_col.to_string();

        Ok(())
    }

    pub fn to_string(auth_obj: AuthJWT) -> String {
        format!(
            "DEFINE auth_jwt [{},{},{}]",
            if auth_obj.active == true {
                "true"
            } else {
                "false"
            },
            auth_obj.field,
            auth_obj.ref_col
        )
    }

    pub fn from_string(auth_obj_str: &str) -> Result<AuthJWT, (usize, String)> {
        let mut current_auth_obj = auth_obj_str
            .split("DEFINE auth_jwt [")
            .collect::<Vec<&str>>();
        if current_auth_obj.len() <= 1 {
            return Err((500, String::from("Error: Invalid auth_jwt string / 1")));
        }

        current_auth_obj = current_auth_obj[1].split("]").collect::<Vec<&str>>();
        if current_auth_obj.len() <= 1 {
            return Err((500, String::from("Error: Invalid auth_jwt string / 2")));
        }

        current_auth_obj = current_auth_obj[0].split(",").collect::<Vec<&str>>();
        if current_auth_obj.len() < 3 {
            return Err((500, String::from("Error: Invalid auth_jwt string / 3")));
        }

        AuthJWT::create(
            current_auth_obj[0] == "true",
            current_auth_obj[1],
            current_auth_obj[2],
        )
    }
}
