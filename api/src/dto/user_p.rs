use sea_orm::sqlx::types::chrono;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub nickname: String,
    pub full_name: String,
    pub disabled: i8,
    pub birthdate: NaiveDate,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchUser{
    pub birthdate: Option<NaiveDate>,
    pub nickname: Option<String>,
    pub full_name: Option<String>,

} 

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse{
    pub nickname: String,
    pub full_name: String,
    pub disabled: i8,
    pub birthdate: NaiveDate,
    pub id:i32,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserParam{
    pub full_name: String,
    pub disabled: i8,
}
impl From<schemas::users::Model> for UserResponse {
fn from(value: schemas::users::Model) -> Self {
    Self {
        nickname:value.nickname,
        full_name:value.fullname,
        disabled:value.disabled,
        birthdate:value.birthdate,
        id:value.id,
    }
}
    
}