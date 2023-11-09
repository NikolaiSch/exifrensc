use entity::prelude::*;
use sea_orm::prelude::*;
use sea_orm::sea_query::TableCreateStatement;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use sea_orm::DbBackend;
use sea_orm::Schema;

pub struct DB {}

impl DB {
    pub async fn new_memory_db() -> Result<DatabaseConnection, anyhow::Error> {
        let db = Database::connect("sqlite::memory:").await?;
        setup_schema(&db).await;
        Ok(db)
    }

    pub async fn new_file_db() -> Result<DatabaseConnection, anyhow::Error> {
        Ok(Database::connect("sqlite://./settings.db?mode=rwc").await?)
    }
}

async fn setup_schema(db: &DatabaseConnection) {
    // Setup Schema helper
    let schema = Schema::new(DbBackend::Sqlite);

    let exif: TableCreateStatement = schema.create_table_from_entity(Exif);
    let file: TableCreateStatement = schema.create_table_from_entity(File);

    db.execute(db.get_database_backend().build(&file))
        .await
        .unwrap();

    db.execute(db.get_database_backend().build(&exif))
        .await
        .unwrap();
}

