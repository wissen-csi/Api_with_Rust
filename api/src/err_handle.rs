use std::error::Error;

use axum::{http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;

pub enum ErrHandleDB {
    Unexpected(Box<dyn Error +Send+Sync>),
    NotFound,
}
impl IntoResponse for ErrHandleDB {
    fn into_response(self) -> axum::response::Response {
        match self {
            ErrHandleDB::Unexpected(e) => {
                eprint!("{}",e.to_string()); 
                (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        },
            ErrHandleDB::NotFound => {
                (StatusCode::NOT_FOUND).into_response()
            }
        }
    }
}
impl From<DbErr> for ErrHandleDB  {
    fn from(value: DbErr) -> Self {
        ErrHandleDB::Unexpected(Box::new(value))
    }
}