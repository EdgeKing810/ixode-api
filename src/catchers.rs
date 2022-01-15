use rocket::{
    catch,
    serde::json::{json, Value},
};

#[catch(401)]
pub fn unauthorized() -> Value {
    json!({
        "status": "401",
        "message": "Error: Missing JWT Bearer Header"
    })
}

#[catch(400)]
pub fn bad_request() -> Value {
    json!({
        "status": "400",
        "message": "Error: Not enough information supplied"
    })
}

#[catch(422)]
pub fn malformed_request() -> Value {
    json!({
        "status": "422",
        "message": "Error: Wrongly formed request"
    })
}
