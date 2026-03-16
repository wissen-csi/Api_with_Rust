use sea_orm::sqlx::types::chrono;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct User {
    pub nickname: String,
    pub fullname: String,
    pub disabled: i8,
    pub birthdate: NaiveDate,
}
#[derive(Serialize)]
pub struct UserResponse{
    pub nickname: String,
    pub fullname: String,
    pub disabled: i8,
    pub birthdate: NaiveDate,
    pub id:i32,
}