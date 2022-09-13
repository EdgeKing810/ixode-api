#[cfg(test)]
#[allow(unused_imports)]
use crate::components::routing::core::core_auth_jwt::AuthJWT;

fn make_core_one() -> crate::components::routing::core::core_auth_jwt::AuthJWT {
    crate::components::routing::core::core_auth_jwt::AuthJWT::create(true, "uid", "users").unwrap()
}

fn get_core_str_one() -> String {
    "DEFINE auth_jwt [true,uid,users]".to_string()
}

#[test]
pub fn run_routing_core_auth_jwt_one() {
    println!("---> Running Routing Core AUTH_JWT One");
    // DEFINE auth_jwt [true,uid,users]

    let auth_jwt = make_core_one();

    assert_eq!(get_core_str_one(), AuthJWT::to_string(auth_jwt));
}

#[test]
pub fn run_routing_core_auth_jwt_two() {
    println!("---> Running Routing Core AUTH_JWT Two");

    // AuthJWT {
    //     active:true,
    //     field: "uid",
    //     ref_col: "users"
    // }

    let auth_jwt_one = AuthJWT::from_string(&get_core_str_one()).unwrap();
    let auth_jwt_two = make_core_one();

    assert_eq!(auth_jwt_two, auth_jwt_one);
}
