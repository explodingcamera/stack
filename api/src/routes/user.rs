use crate::{openapi, Db, User};

#[openapi]
#[post("/users/login")]
pub fn login(db: &Db) -> &'static str {
    "Hello, world!"
}

#[openapi]
#[get("/users/me")]
pub fn me(db: &Db, user: User) -> &'static str {
    "Hello, world!"
}
