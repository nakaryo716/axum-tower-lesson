use axum::{routing::get, Router};
use middle::AppLayer;

mod middle;
#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let app = Router::new()
        .layer(AppLayer)
        .route("/nolayer", get(|| async {"no layerd page".to_string()}));


    axum::serve(listener, app).await.unwrap();
}
