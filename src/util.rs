use lazy_static::lazy_static;
use rocket::http::{ContentType, Status};

lazy_static! {
    pub static ref UPLOAD_PATH: String =
        std::env::var("UPLOAD_PATH").expect("UPLOAD_PATH must be set");
    pub static ref ADMIN_PASSWORD: String =
        std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
}

pub type JsonResponse = (Status, (ContentType, String));

#[macro_export]
macro_rules! make_json_response {
    ($status:expr, $message:expr, $data:expr) => {
        (
            match rocket::http::Status::from_code($status as u16) {
                Some(code) => code,
                None => rocket::http::Status::new($status as u16)
            },
            (
                rocket::http::ContentType::JSON,
                serde_json::json!({
                    "status": $status as i32,
                    "message": $message,
                    "data": $data,
                })
                .to_string()
            )
        )
    };
    ($status:expr, $message:expr) => {
        (
            match rocket::http::Status::from_code($status as u16) {
                Some(code) => code,
                None => rocket::http::Status::new($status as u16)
            },
            (
                rocket::http::ContentType::JSON,
                serde_json::json!({
                    "status": $status as i32,
                    "message": $message,
                })
                .to_string()
            )
        )
    }
}

#[macro_export]
macro_rules! unwrap_or_return_result {
    ($r:expr, $s:expr) => {
        match $r {
            Ok(r) => r,
            Err(e) => {
                warn!("Unwrapped on error {} (error {})", e, $s);
                return None;
            }
        }
    };
}

#[macro_export]
macro_rules! unwrap_or_return_option {
    ($o:expr, $s:expr) => {
        match $o {
            Some(r) => r,
            None => {
                warn!("Unwrapped on None (error {})", $s);
                return None;
            }
        }
    };
}
