use crate::{db_connection::PgPool, ManagedPool};
use diesel::prelude::*;
use rand::Rng;
use rocket::{
    http::{Cookie, CookieJar, Status},
    outcome::Outcome,
    request::{self, FromRequest, Request},
    serde::json::Json,
    State,
};
use serde::Deserialize;

pub struct AdminUser;

#[derive(Debug, PartialEq)]
pub enum AdminError {
    InvalidCredentials,
    MissingCookie,
    InternalServerError,
    ExpiredSession,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminUser {
    type Error = AdminError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let managed_pool = match request.guard::<&State<ManagedPool>>().await {
            Outcome::Success(pool) => pool,
            Outcome::Failure(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    AdminError::InternalServerError,
                ))
            }
            Outcome::Forward(_) => {
                return Outcome::Failure((
                    Status::InternalServerError,
                    AdminError::InternalServerError,
                ))
            }
        };
        let pool: &PgPool = &managed_pool.0;

        let cookies = request.cookies();
        let session_id = match cookies.get_private("session_id") {
            Some(c) => c.value().to_owned(),
            None => return Outcome::Failure((Status::Unauthorized, AdminError::MissingCookie)),
        };

        let connection = match pool.get() {
            Ok(c) => c,
            Err(e) => {
                warn!("Failed to get connection from pool with error: {}", e);
                return Outcome::Failure((
                    Status::InternalServerError,
                    AdminError::InternalServerError,
                ));
            }
        };

        let session = match crate::schema::sessions::dsl::sessions
            .filter(crate::schema::sessions::dsl::session_id.eq(&session_id))
            .first::<crate::model::Session>(&connection)
        {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to get session with error: {}", e);
                if e == diesel::NotFound {
                    return Outcome::Failure((
                        Status::Unauthorized,
                        AdminError::InvalidCredentials,
                    ));
                }
                return Outcome::Failure((
                    Status::InternalServerError,
                    AdminError::InternalServerError,
                ));
            }
        };

        if session.expires_at < chrono::Utc::now().naive_utc() {
            Outcome::Failure((Status::Unauthorized, AdminError::ExpiredSession))
        } else {
            Outcome::Success(AdminUser)
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct LoginData {
    pub password: String,
}

#[post("/login", data = "<data>")]
pub async fn login(
    data: Json<LoginData>,
    pool: &State<ManagedPool>,
    cookies: &CookieJar<'_>,
) -> Status {
    if data.password == crate::util::ADMIN_PASSWORD.as_str() {
        let session_id = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(255)
            .map(char::from)
            .collect::<String>();
        let session = crate::model::SessionNoID {
            session_id: session_id.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at: chrono::Utc::now().naive_utc() + chrono::Duration::hours(6),
        };

        let pool: &PgPool = &pool.0;
        let connection = match pool.get() {
            Ok(c) => c,
            Err(e) => {
                warn!("Failed to get connection from pool with error: {}", e);
                return Status::InternalServerError;
            }
        };
        match diesel::insert_into(crate::schema::sessions::dsl::sessions)
            .values(&session)
            .execute(&connection)
        {
            Ok(_) => (),
            Err(e) => {
                warn!("Failed to insert session with error: {}", e);
                return Status::InternalServerError;
            }
        }
        cookies.add_private(Cookie::new("session_id", session_id));
        Status::Ok
    } else {
        Status::Unauthorized
    }
}
