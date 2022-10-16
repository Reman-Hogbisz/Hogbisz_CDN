use super::model::*;
use crate::model::ServerResponse;
use gloo_net::http::Request;
use web_sys::File;

pub enum FileUploadError {
    InternalError,
    DeserializeError,
    ResponseError(String),
    AuthError,
}

pub async fn upload_file(file: File) -> Result<FileUploadData, FileUploadError> {
    let multipart_form = match web_sys::FormData::new() {
        Ok(form) => match form.append_with_blob("file", &file) {
            Ok(_) => match form.append_with_str("file_name", &file.name()) {
                Ok(_) => form,
                Err(e) => {
                    gloo::console::error!(format!(
                        "Failed to append file_name to form with error \"{:?}\"",
                        e
                    ));
                    return Err(FileUploadError::InternalError);
                }
            },
            Err(e) => {
                gloo::console::error!(format!(
                    "Failed to append file to form with error \"{:?}\"",
                    e
                ));
                return Err(FileUploadError::InternalError);
            }
        },
        Err(e) => {
            gloo::console::error!(format!("Failed to create form data with error \"{:?}\"", e));
            return Err(FileUploadError::InternalError);
        }
    };

    let initial_result = Request::post("/api/upload")
        .body(&multipart_form)
        .send()
        .await;

    match initial_result {
        Ok(response) => match response.status() {
            200 => match response.json::<ServerResponse<FileUploadData>>().await {
                Ok(json) => Ok(json.data.unwrap()),
                Err(e) => {
                    gloo::console::error!(format!(
                        "Failed to deserialize response with error \"{:?}\"",
                        e
                    ));
                    Err(FileUploadError::DeserializeError)
                }
            },
            401 => Err(FileUploadError::AuthError),
            _ => {
                gloo::console::error!(format!(
                    "Failed to upload file with status code \"{}\"",
                    response.status()
                ));
                Err(FileUploadError::InternalError)
            }
        },
        Err(e) => {
            gloo::console::error!(format!("Failed to send request with error \"{:?}\"", e));
            Err(FileUploadError::InternalError)
        }
    }
}
