use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(Table::create().table(Users::Table).if_not_exists().col(pk_auto(Users::Id)).col(string_len_uniq(Users::Nickname, 100)).col(string_len(Users::Fullname,200)).col(boolean(Users::Disabled)).col(date_time(Users::Datecreation)).col(date(Users::Birthdate)).to_owned()).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Fullname,
    Nickname,
    Disabled,
    Datecreation,
    Birthdate,
}
