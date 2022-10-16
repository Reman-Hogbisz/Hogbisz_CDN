use gloo_net::http::Request;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct LoginData {
    pub password: String,
}

pub enum TryLoginError {
    IncorrectPassword,
    InternalError,
    SerializeError,
}

pub async fn try_login(password: String) -> Result<(), TryLoginError> {
    let login_info = LoginData { password };
    let json_body = match serde_json_wasm::to_string(&login_info) {
        Ok(body) => body,
        Err(e) => {
            gloo::console::error!(format!("Failed to serialize login info (\"{}\")", e));
            return Err(TryLoginError::SerializeError);
        }
    };
    match Request::post("/api/login")
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
    {
        Ok(response) => {
            match response.status() {
                200 => Ok(()),
                401 => Err(TryLoginError::IncorrectPassword),
                _ => {
                    gloo::console::error!(format!("Unexpected status code: {}", response.status()));
                    Err(TryLoginError::InternalError)
                }
            }
        }
        Err(e) => {
            gloo::console::error!(format!("Failed to send login request (\"{}\")", e));
            Err(TryLoginError::InternalError)
        }
    }
}
