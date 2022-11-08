use rocket::http::uri::{fmt::Path, Segments};
use rocket::request::{FromParam, FromSegments};

pub struct CompleteRoute {
    r: String,
}

impl<'r> FromParam<'r> for CompleteRoute {
    type Error = &'r str;

    fn from_param(r: &'r str) -> Result<Self, Self::Error> {
        if !r.chars().all(|c| {
            c.is_ascii_alphanumeric() || c == '&' || c == '!' || c == '#' || c == '-' || c == '_'
        }) {
            return Err(r);
        }

        Ok(Self { r: r.to_string() })
    }
}

impl<'r> FromSegments<'r> for CompleteRoute {
    type Error = String;

    fn from_segments(segments: Segments<'r, Path>) -> Result<Self, Self::Error> {
        let mut r = String::new();
        for segment in segments {
            r = format!("{}/{}", r, segment);
        }

        if !r.clone().chars().all(|c| {
            c.is_ascii_alphanumeric()
                || c == '&'
                || c == '!'
                || c == '#'
                || c == '-'
                || c == '_'
                || c == '?'
                || c == '/'
        }) {
            return Err(r.clone());
        }

        Ok(Self { r })
    }
}
