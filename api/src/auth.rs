use okapi::openapi3::{SecurityScheme, SecuritySchemeData};
use ory_kratos_client::{apis::v0alpha2_api::to_session, models::Session};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use thiserror::Error;

fn get_auth_scheme(required: bool) -> RequestHeaderInput {
    let mut requirements = schemars::Map::new();
    if required {
        requirements.insert("Session Token".to_owned(), vec![]);
    };
    RequestHeaderInput::Security(
        "Session Token".to_owned(),
        SecurityScheme {
            description: Some("Authentication using ORY Kratos".to_string()),
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_string(),
                location: "header".to_string(),
            },
            extensions: Default::default(),
        },
        requirements,
    )
}

pub struct OptionalUser {
    pub session: Option<Session>,
}

pub struct User {
    pub session: Session,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("missing sessions cookie")]
    NoSessionCookie,
    #[error("internal error")]
    KratosError,
}

async fn request_to_session(request: &Request<'_>) -> Result<Session, UserError> {
    let cookie = request
        .cookies()
        .get("Authorization")
        .ok_or(UserError::NoSessionCookie)?;
    let session_token = cookie.value();
    let session = to_session(&crate::config::KRATOS_CONFIG, Some(session_token), None).await;
    match session {
        Ok(session) => Ok(session),
        Err(_) => Err(UserError::KratosError),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request_to_session(req).await {
            Ok(session) => Outcome::Success(User { session }),
            Err(e) => Outcome::Failure((Status::Unauthorized, e)),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for OptionalUser {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request_to_session(req).await {
            Ok(session) => Outcome::Success(OptionalUser {
                session: Some(session),
            }),
            Err(_) => Outcome::Success(OptionalUser { session: None }),
        }
    }
}

impl<'r> OpenApiFromRequest<'r> for OptionalUser {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(get_auth_scheme(false))
    }
}

impl<'r> OpenApiFromRequest<'r> for User {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(get_auth_scheme(true))
    }
}
