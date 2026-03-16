use sea_orm::sqlx::types::chrono;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub nickname: String,
    pub fullname: String,
    pub disabled: i8,
    pub datecreation: NaiveDate,
    pub birthdate: NaiveDate,
}