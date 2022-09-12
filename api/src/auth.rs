use okapi::openapi3::{SecurityScheme, SecuritySchemeData};
use ory_kratos_client::{apis::v0alpha2_api::to_session, models::Session};
use rocket::{
    http::Status,
    request::{self, FromRequest, Outcome, Request},
};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use thiserror::Error;

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = UserError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let session_token = match req.headers().get_one("x-session-token") {
            Some(header) => header,
            None => return Outcome::Failure((Status::Unauthorized, UserError::NoSessionCookie)),
        };

        let session =
            match to_session(&crate::config::KRATOS_CONFIG, Some(session_token), None).await {
                Ok(session) => session,
                Err(_) => return Outcome::Failure((Status::Unauthorized, UserError::KratosError)),
            };

        let user = User { session };
        Outcome::Success(user)
    }
}

impl<'r> OpenApiFromRequest<'r> for User {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        let mut requirements = schemars::Map::new();
        requirements.insert("Session Token".to_owned(), vec![]);

        Ok(RequestHeaderInput::Security(
            "Session Token".to_owned(),
            SecurityScheme {
                data: SecuritySchemeData::ApiKey {
                    name: "x-session-token".to_owned(),
                    location: "header".to_owned(),
                },
                description: Some("Used to authenticate as a user.".to_owned()),
                extensions: schemars::Map::new(),
            },
            requirements,
        ))
    }
}
