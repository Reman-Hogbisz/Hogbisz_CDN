use super::model::*;
use crate::model::ServerResponse;
use gloo_net::http::Request;
use web_sys::File;

pub enum FileUploadError {
    InternalError,
    DeserializeError,
    ResponseError(String),
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
        Ok(response) => match response.json::<ServerResponse<FileUploadData>>().await {
            Ok(json) => match json.status {
                200 => Ok(json.data.unwrap()),
                _ => Err(FileUploadError::ResponseError(json.message)),
            },
            Err(e) => {
                gloo::console::error!(format!(
                    "Failed to deserialize response with error \"{:?}\"",
                    e
                ));
                return Err(FileUploadError::DeserializeError);
            }
        },
        Err(e) => {
            gloo::console::error!(format!("Failed to send request with error \"{:?}\"", e));
            return Err(FileUploadError::InternalError);
        }
    }
}
