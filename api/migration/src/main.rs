use sea_orm_migration::prelude::*;

#[tokio::main]
async fn main() {
    //  Setting `DATABASE_URL` environment variable
    // let key = "DATABASE_URL";
    //     std::env::set_var(key, database_url);
    cli::run_cli(migration::Migrator).await;
}
