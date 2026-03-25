use api::{
    db::{self, AppContext},
    dto::{
        Gurrupleta69,
        user_p::{User, UserParam, UserResponse},
    },
    err_handle::ErrHandleDB,
};
use axum::{
    Json,
    extract::{Path, Query, State}, http::{StatusCode},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, Database, EntityTrait, PaginatorTrait, QueryOrder,
    sqlx::types::chrono,
};
use std::{env, net::SocketAddr};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let databsae_url = env::var("DATABASE_URL").expect("not found");
    let conn = Database::connect(databsae_url).await.expect("falled");
    let ctx = db::AppContext { conn };
    let port = 8081;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            panic!("listener error port: {} error: {}", addr, e);
        });
    let router = axum::Router::new()
        .route("/api/post", axum::routing::post(create_user))
        .route("/api/get", axum::routing::get(get_user))
        .route("/api/search/{id}", axum::routing::get(search_user))
        .route("/api/put", axum::routing::put(user_put))
        .with_state(ctx);
    axum::serve(listener, router).await.unwrap_or_else(|err| {
        panic!("{}", err);
    });
}
pub async fn user_put(State(ctx):State<AppContext>, Json(param):Json<UserParam>) -> Result<StatusCode,ErrHandleDB>
{
    Ok(StatusCode::CREATED)
}
pub async fn search_user(
    Path(id): Path<i32>,
    State(cxt): State<AppContext>,
) -> Result<Json<UserResponse>, ErrHandleDB> {
    let x = schemas::users::Entity::find_by_id(id)
        .one(&cxt.conn)
        .await?
        .ok_or_else(|| ErrHandleDB::NotFound)?;

    Ok(Json(x.into()))
}
pub async fn create_user(
    State(ctx): State<AppContext>,
    Json(user): Json<User>,
) -> Result<StatusCode, ErrHandleDB> {
    schemas::users::ActiveModel {
        id: ActiveValue::NotSet,
        nickname: ActiveValue::Set(user.nickname),
        fullname: ActiveValue::Set(user.full_name),
        disabled: ActiveValue::Set(user.disabled),
        birthdate: ActiveValue::Set(user.birthdate),
        datecreation: ActiveValue::Set(chrono::Utc::now().naive_utc()),
    }
    .insert(&ctx.conn)
    .await?;
    Ok(StatusCode::CREATED)
}
pub async fn get_user(
    State(ctx): State<AppContext>,
    Query(gurrupleta): Query<Gurrupleta69>,
) -> Result<Json<Vec<UserResponse>>, ErrHandleDB> {
    let x = schemas::users::Entity::find()
        .order_by_desc(schemas::users::Column::Id)
        .paginate(&ctx.conn, gurrupleta.la_maleta)
        .fetch_page(gurrupleta.pogina)
        .await?;
    let y = x
        .into_iter()
        .map(|model| model.into())
        .collect();
    Ok(Json(y))
}
