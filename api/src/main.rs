use std::net::SocketAddr;
use axum::response::IntoResponse;


#[tokio::main] 
async  fn main() {
    let port = 8081;
    let addr = SocketAddr::from(([0,0,0,0],port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap_or_else(|e| {
        panic!("listener error port: {} error: {}",addr,e);
    });
    let test_router = axum::Router::new().route("/api", axum::routing::get(m));
    axum::serve(listener,test_router).await.unwrap_or_else(|err|{
        panic!("{}",err);
    });
}
pub async fn m() -> impl IntoResponse{
    format!("test")
}
