use async_trait::async_trait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Intents::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Intents::Id)
                            .big_integer()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Intents::Creator).string().not_null())
                    .col(ColumnDef::new(Intents::InputToken).string().not_null())
                    .col(ColumnDef::new(Intents::InputAmount).string().not_null())
                    .col(ColumnDef::new(Intents::OutputToken).string().not_null())
                    .col(ColumnDef::new(Intents::MinOutputAmount).string().not_null())
                    .col(ColumnDef::new(Intents::Deadline).big_integer().not_null())
                    .col(ColumnDef::new(Intents::Status).small_integer().not_null())
                    .col(
                        ColumnDef::new(Intents::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Intents::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for fast lookup by creator
        manager
            .create_index(
                Index::create()
                    .name("idx_intents_creator")
                    .table(Intents::Table)
                    .col(Intents::Creator)
                    .to_owned(),
            )
            .await?;

        // Index for filtering by status
        manager
            .create_index(
                Index::create()
                    .name("idx_intents_status")
                    .table(Intents::Table)
                    .col(Intents::Status)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Intents::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Intents {
    Table,
    Id,
    Creator,
    InputToken,
    InputAmount,
    OutputToken,
    MinOutputAmount,
    Deadline,
    Status,
    CreatedAt,
    UpdatedAt,
}
