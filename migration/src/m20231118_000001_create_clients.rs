use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Client::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Client::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Client::Uuid).string().not_null())
                    .col(ColumnDef::new(Client::Name).string().not_null())
                    .col(ColumnDef::new(Client::Description).string())
                    .col(ColumnDef::new(Client::Secret).string().not_null())
                    .col(ColumnDef::new(Client::RedirectUris).string().not_null())
                    .col(ColumnDef::new(Client::GrantTypes).string().not_null())
                    .col(ColumnDef::new(Client::ResponseTypes).string().not_null())
                    .col(ColumnDef::new(Client::Scope).string().not_null())
                    .col(
                        ColumnDef::new(Client::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Client::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .table(Client::Table)
                    .name("idx_clients_uuid")
                    .col(Client::Uuid)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_index(
                Index::drop()
                    .table(Client::Table)
                    .name("idx_clients_uuid")
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Client::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Client {
    #[sea_orm(iden = "clients")]
    Table,
    Id,
    Uuid,
    Name,
    Description,
    Secret,
    RedirectUris,
    GrantTypes,
    ResponseTypes,
    Scope,
    CreatedAt,
    UpdatedAt,
}
