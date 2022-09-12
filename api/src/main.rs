use anyhow::{Context, Result};

#[macro_use]
extern crate rocket;
use rocket::serde::json::{json, Value};
use rocket::State;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

use prisma::PrismaClient;

mod auth;
mod config;
mod cors;
mod prisma;
mod routes;

pub type Db = State<PrismaClient>;
pub use auth::User;
pub use rocket_okapi::openapi;

#[openapi]
#[get("/")]
fn index() -> &'static str {
    ":)"
}

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn get_docs() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "/api/v1/openapi.json".to_string(),
        ..Default::default()
    }
}

#[rocket::main]
async fn main() -> Result<()> {
    let config = rocket::Config {
        port: 7777,
        ..rocket::Config::debug_default()
    };

    let client: PrismaClient = prisma::new_client()
        .await
        .with_context(|| "Failed to create Prisma client")?;

    let server = rocket::build()
        .manage(client)
        .attach(cors::Cors)
        .configure(config)
        .mount("/", routes![index])
        .mount(
            "/api/v1",
            openapi_get_routes![
                index,
                // users
                routes::user::login,
                routes::user::me,
            ],
        )
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .register("/", catchers![not_found]);

    server
        .launch()
        .await
        .map(|_| ())
        .with_context(|| "Failed to launch rocket server")
}
