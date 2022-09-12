use rocket::serde::{Deserialize, Serialize};

use super::{sub_function_list::FunctionList, sub_ref_data::RefData};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    pub id: FunctionList,
    pub params: Vec<RefData>,
}

impl Function {
    pub fn create(flist_txt: &str) -> Function {
        Function {
            id: FunctionList::from(flist_txt),
            params: vec![],
        }
    }

    pub fn update_id(function: &mut Function, flist_txt: &str) {
        function.id = FunctionList::from(flist_txt);
    }

    pub fn add_param(function: &mut Function, param: RefData) {
        function.params.push(param);
    }

    pub fn remove_param(function: &mut Function, param_index: u32) -> Result<(), (usize, String)> {
        let mut updated_params = Vec::<RefData>::new();
        if param_index >= function.params.len() as u32 {
            return Err((
                400,
                String::from("Error: Index goes over the amount of params present"),
            ));
        }

        for n in 0..function.params.len() {
            if n as u32 != param_index {
                updated_params.push(function.params[n].clone());
            }
        }

        function.params = updated_params;

        Ok(())
    }

    pub fn set_params(function: &mut Function, params: Vec<RefData>) {
        function.params = params;
    }

    pub fn to_string(function: Function) -> String {
        let mut params_str = String::new();
        for param in function.params {
            params_str = format!(
                "{}{}{}",
                params_str,
                if params_str.chars().count() > 1 {
                    ">"
                } else {
                    ""
                },
                RefData::to_string(param.clone())
            );
        }

        format!("{}={}", FunctionList::to(function.id), params_str)
    }

    pub fn from_string(function_str: &str) -> Result<Function, (usize, String)> {
        let current_function = function_str.split("=").collect::<Vec<&str>>();
        if current_function.len() <= 1 {
            return Err((500, String::from("Error: Invalid function_str string / 1")));
        }

        let flist_txt = current_function[0];
        let mut params = Vec::<RefData>::new();
        let params_list_str = current_function[1..].join("=");
        let params_list = params_list_str.trim().split(">").collect::<Vec<&str>>();

        for p_str in params_list {
            if p_str.len() < 1 {
                continue;
            }

            match RefData::from_string(p_str) {
                Ok(p) => params.push(p),
                Err(e) => {
                    return Err((
                        500,
                        format!("Error: Invalid function_str string / 2: {}", e.1),
                    ))
                }
            }
        }

        let mut function = Function::create(flist_txt);
        Function::set_params(&mut function, params);

        Ok(function)
    }
}
