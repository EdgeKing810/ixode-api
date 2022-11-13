use rocket::serde::{Deserialize, Serialize};

use crate::{
    components::constraint_property::ConstraintProperty,
    utils::{constraint::auto_fetch_all_constraints, mapping::auto_fetch_all_mappings},
};

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
        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "auth_jwt", "field", field) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        auth_obj.field = final_value;

        Ok(())
    }

    pub fn update_ref_col(auth_obj: &mut AuthJWT, ref_col: &str) -> Result<(), (usize, String)> {
        let mappings = auto_fetch_all_mappings();
        let all_constraints = match auto_fetch_all_constraints(&mappings) {
            Ok(c) => c,
            Err(e) => return Err((500, e)),
        };
        let final_value =
            match ConstraintProperty::validate(&all_constraints, "auth_jwt", "ref_col", ref_col) {
                Ok(v) => v,
                Err(e) => return Err(e),
            };

        auth_obj.ref_col = final_value;

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
            return Err((500, String::from("Invalid auth_jwt (at declaration start)")));
        }

        current_auth_obj = current_auth_obj[1].split("]").collect::<Vec<&str>>();
        if current_auth_obj.len() <= 1 {
            return Err((500, String::from("Invalid auth_jwt (at declaration end)")));
        }

        current_auth_obj = current_auth_obj[0].split(",").collect::<Vec<&str>>();
        if current_auth_obj.len() < 3 {
            return Err((500, String::from("Invalid auth_jwt (in format)")));
        }

        AuthJWT::create(
            current_auth_obj[0] == "true",
            current_auth_obj[1],
            current_auth_obj[2],
        )
    }
}
