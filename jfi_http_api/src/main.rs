use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route(
            "/fund/jerryIndex/fundCode/:fund_code",
            get(jfi_http_api::get_jerry_index_by_fund_code),
        )
        .route(
            "/fund/baiduIndex/keyword/:keyword",
            get(jfi_http_api::get_baidu_index_by_keyword),
        )
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    println!("port {}", port);

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    println!("address on {}", address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
