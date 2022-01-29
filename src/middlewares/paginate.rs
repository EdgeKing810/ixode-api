use crate::components::{config::Config, user::User};

pub fn paginate_configs(data: Vec<Config>, limit: usize, offset: usize) -> Vec<Config> {
    if limit == 0 {
        return data;
    }

    let mut new_configs = Vec::<Config>::new();

    let start = limit * offset;
    let end = start + limit;
    let length = data.len();

    for n in start..end {
        if n >= length {
            break;
        }

        new_configs.push(data[n].clone());
    }

    new_configs
}

pub fn paginate_users(data: Vec<User>, limit: usize, offset: usize) -> Vec<User> {
    if limit == 0 {
        return data;
    }

    let mut new_users = Vec::<User>::new();

    let start = limit * offset;
    let end = start + limit;
    let length = data.len();

    for n in start..end {
        if n >= length {
            break;
        }

        new_users.push(data[n].clone());
    }

    new_users
}
