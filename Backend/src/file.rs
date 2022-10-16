use crate::{make_json_response, model::*, util::JsonResponse, ManagedPool};
use diesel::prelude::*;
use rand::Rng;
use rocket::{form::Form, fs::TempFile, http::Status, State};
use rocket_seek_stream::SeekStream;

#[derive(FromForm)]
pub struct FileUploadForm<'r> {
    file: TempFile<'r>,
    file_name: String,
}

#[post("/upload", data = "<data>")]
pub async fn upload_file(
    mut data: Form<FileUploadForm<'_>>,
    pool: &State<ManagedPool>,
    _admin_guard: crate::auth::AdminUser,
) -> JsonResponse {
    let user_file_name = data
        .file_name
        .replace("..", "")
        .replace("/", "")
        .replace("\\", "");

    let mut file_name;
    let mut secret;
    loop {
        secret = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(32)
            .map(char::from)
            .collect::<String>();
        file_name = format!(
            "{}/{}_{}",
            crate::util::UPLOAD_PATH.as_str(),
            secret,
            user_file_name
        );
        if let Some(_) = std::fs::File::open(&file_name).ok() {
            continue;
        } else {
            break;
        }
    }
    match data.file.copy_to(&file_name).await {
        Ok(_) => (),
        Err(e) => {
            warn!("Failed to move uploaded file with error: {}", e);
            return make_json_response!(500, "Internal Server Error");
        }
    }

    let connection = match pool.0.get() {
        Ok(c) => c,
        Err(e) => {
            match std::fs::remove_file(&file_name) {
                Ok(_) => (),
                Err(e) => {
                    warn!("Failed to remove file \"{}\" with error: {}", file_name, e);
                }
            }
            warn!("Failed to get connection from pool with error: {}", e);
            return make_json_response!(500, "Internal Server Error");
        }
    };

    match diesel::insert_into(crate::schema::file_uploads::table)
        .values(crate::model::FileUploadNoID {
            secret,
            name: user_file_name,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            deleted_at: None,
        })
        .get_result::<FileUpload>(&connection)
    {
        Ok(f) => make_json_response!(200, "OK", f),
        Err(e) => {
            warn!("Failed to insert file upload with error: {}", e);
            make_json_response!(500, "Internal Server Error")
        }
    }
}

#[get("/uploads/<secret>/<file_name>")]
pub async fn get_file(secret: String, file_name: String) -> Result<SeekStream, Status> {
    let file_name = format!(
        "{}/{}_{}",
        crate::util::UPLOAD_PATH.as_str(),
        secret,
        file_name
    );
    match SeekStream::from_path(file_name).await {
        Ok(s) => Ok(s),
        Err(e) => {
            warn!("Failed to get file with error: {}", e);
            match e.kind() {
                std::io::ErrorKind::NotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}
