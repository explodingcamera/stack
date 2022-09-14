use crate::{openapi, Db, User};

#[openapi]
#[get("/users/me")]
pub fn me(db: &Db, user: User) -> &'static str {
    "Hello, world!"
}

#[openapi]
#[post("/users/profile")]
pub fn update_profile(db: &Db, user: User) -> &'static str {
    "Hello, world!"
}
