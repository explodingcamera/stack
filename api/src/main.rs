#[macro_use]
extern crate rocket;

mod prisma;
use prisma::PrismaClient;
use prisma_client_rust::NewClientError;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    let client: Result<PrismaClient, NewClientError> = prisma::new_client().await;
    let rocket = rocket::build().mount("/", routes![index]);
}
