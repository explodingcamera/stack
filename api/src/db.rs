use anyhow::{Context, Result};
use sea_orm::{sea_query::TableCreateStatement, ConnectionTrait, DbBackend, DbConn, Schema};

pub async fn setup_schema(db: &DbConn) -> Result<()> {
    // Setup Schema helper
    let schema = Schema::new(DbBackend::Sqlite);

    // Derive from Entity
    let stmt: TableCreateStatement = schema.create_table_from_entity(entity::profile::Entity);

    // Execute create table statement
    db.execute(db.get_database_backend().build(&stmt))
        .await
        .with_context(|| "failed to create table")?;
    Ok(())
}
