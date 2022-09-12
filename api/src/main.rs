#[macro_use]
extern crate rocket;

mod prisma;
use anyhow::{Context, Result};
use prisma::PrismaClient;

use rocket::State;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use rocket_okapi::{openapi, openapi_get_routes};

#[openapi]
#[get("/")]
fn index(_db: &State<PrismaClient>) -> &'static str {
    "Hello, world!"
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

    rocket::build()
        .manage(client)
        .mount("/api/v1", openapi_get_routes![index])
        .mount("/swagger", make_swagger_ui(&get_docs()))
        .configure(config)
        .launch()
        .await
        .map(|_| ())
        .with_context(|| "Failed to launch rocket server")
}
