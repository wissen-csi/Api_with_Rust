use std::{env, net::SocketAddr};
use api::{db::{self, AppContext}, dto::user_p::User, err_handle::ErrHandleDB};
use axum::{Json, extract::{Path, State}, response::IntoResponse};
use sea_orm::{ActiveModelTrait, ActiveValue, Database, EntityOrSelect, EntityTrait, sqlx::types::chrono};




#[tokio::main] 
async  fn main() {
    dotenvy::dotenv().ok();
    let databsae_url = env::var("DATABASE_URL").expect("not found");
    let conn = Database::connect(databsae_url).await.expect("falled");
    let ctx = db::AppContext {conn};
    let port = 8081;
    let addr = SocketAddr::from(([0,0,0,0],port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap_or_else(|e| {
        panic!("listener error port: {} error: {}",addr,e);
    });
    let router = axum::Router::new().route("/api/post", axum::routing::post(axum::routing::post(create_user))).route("/api/get/{id}", axum::routing::get(get_user)).with_state(ctx);
    axum::serve(listener,router).await.unwrap_or_else(|err|{
        panic!("{}",err);
    });
}
pub async fn create_user(State(ctx): State<AppContext>, Json(user): Json<User>) -> Result<(),ErrHandleDB> {    
    let user_new = schemas::users::ActiveModel {
        id: ActiveValue::NotSet,
        nickname: ActiveValue::Set(user.nickname),
        fullname: ActiveValue::Set(user.fullname),
        disabled: ActiveValue::Set(user.disabled),
        birthdate: ActiveValue::Set(user.birthdate),
        datecreation: ActiveValue::Set(chrono::Utc::now().naive_utc()),
    }.insert(&ctx.conn).await?;
    Ok(())

}
pub async fn get_user(Path(id):Path<i32>,State(ctx):State<AppContext>) -> impl IntoResponse{
    let row = schemas::users::Entity::find_by_id(id);
    //lack this method
}
