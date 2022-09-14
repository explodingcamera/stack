use crate::config::SUPERTOKENS_CONFIG;
use crate::{openapi, Db, User};

use supertokens::apis::passwordless_recipe_api::passwordless_start_sign_in;
use supertokens::apis::passwordless_recipe_api::passwordless_try_use_code;
use supertokens::models::{PasswordlessStartSignInRequest, PasswordlessTryUseCodeRequest};

#[openapi]
#[post("/auth/login/passwordless")]
pub async fn login_passwordless(db: &Db) -> &'static str {
    let device_id = "device_id".to_string();
    let email = "email".to_string();

    let res = passwordless_start_sign_in(
        &SUPERTOKENS_CONFIG,
        "",
        None,
        None,
        Some(PasswordlessStartSignInRequest {
            email,
            phone_number: "".to_owned(),
            user_input_code: None,
            device_id,
        }),
    )
    .await
    .unwrap();

    // Always required
    let auth_session_id = res.pre_auth_session_id;

    // Magic link flow
    // let magic_link_code = res.link_code;

    // OTP FLOW
    let otp_code = res.user_input_code;
    let lifetime = res.code_lifetime;

    "Hello, world!"
}

#[openapi]
#[post("/auth/login/passwordless/verify")]
pub async fn login_passwordless_verify(db: &Db) -> &'static str {
    let auth_session_id = "".to_owned();
    let device_id = "".to_owned();
    let code = "".to_owned();

    let res = passwordless_try_use_code(
        &SUPERTOKENS_CONFIG,
        "",
        None,
        None,
        Some(PasswordlessTryUseCodeRequest {
            pre_auth_session_id: Some(auth_session_id),
            link_code: "".to_string(),
            device_id,
            user_input_code: code,
        }),
    )
    .await
    .unwrap();

    "Hello, world!"
}
