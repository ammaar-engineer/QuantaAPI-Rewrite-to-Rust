use sea_orm_migration::{prelude::*, schema::*};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20261109_131724_new_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
        manager
            .create_table(
                Table::create()
                .table("users")
                .if_not_exits()
                .col(ColumnDef::new("id").uuid().not_null().primary_key())
                .col(ColumnDef::new("name").string().not_null())
                .col(ColumnDef::new("picture").string().not_null())
                .col(ColumnDef::new("total_endpoints").integer().not_null().default(0))
            )
            .await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();
    }
}
