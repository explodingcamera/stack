use okapi::openapi3::{SecurityScheme, SecuritySchemeData};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use supertokens::models::{Session, VerifySessionRequest};
use thiserror::Error;

use crate::config::SUPERTOKENS_CONFIG;

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

#[derive(Default)]
pub struct OptionalUser {
    pub session: Option<Session>,
    pub id: Option<String>,
}

pub struct User {
    pub session: Session,
    pub id: String,
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
    let session = supertokens::apis::session_recipe_api::verify_session(
        &SUPERTOKENS_CONFIG,
        "",
        None,
        None,
        Some(VerifySessionRequest {
            access_token: Some(session_token.to_string()),
            ..Default::default()
        }),
    )
    .await;

    // let session = to_session(&crate::config::KRATOS_CONFIG, Some(session_token), None).await;
    match session {
        Ok(session) => match session.session {
            Some(session) => Ok(*session),
            None => Err(UserError::KratosError),
        },
        Err(_) => Err(UserError::KratosError),
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match request_to_session(req).await {
            Ok(session) => match session.user_id.clone() {
                Some(id) => Outcome::Success(User { session, id }),
                None => Outcome::Failure((Status::Unauthorized, UserError::KratosError)),
            },
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
                id: session.user_id.clone(),
                session: Some(session),
            }),
            Err(_) => Outcome::Success(OptionalUser::default()),
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
