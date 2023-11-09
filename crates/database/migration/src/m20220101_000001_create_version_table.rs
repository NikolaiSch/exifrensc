use sea_orm_migration::prelude::*;

use std::time;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let major: u32 = env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap();
        let minor: u32 = env!("CARGO_PKG_VERSION_MINOR").parse().unwrap();
        let patch: u32 = env!("CARGO_PKG_VERSION_PATCH").parse().unwrap();

        manager
            .create_table(
                Table::create()
                    .table(Release::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Release::Major)
                            .integer()
                            .not_null()
                            .default(major),
                    )
                    .col(
                        ColumnDef::new(Release::Minor)
                            .integer()
                            .not_null()
                            .default(minor),
                    )
                    .col(
                        ColumnDef::new(Release::Patch)
                            .integer()
                            .not_null()
                            .default(patch),
                    )
                    .col(
                        ColumnDef::new(Release::Date)
                            .timestamp()
                            .not_null()
                            .default(
                                time::SystemTime::now()
                                    .duration_since(time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            ),
                    )
                    .primary_key(
                        Index::create()
                            .col(Release::Major)
                            .col(Release::Minor)
                            .col(Release::Patch),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Release::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Release {
    Table,
    Major,
    Minor,
    Patch,
    Date,
}
