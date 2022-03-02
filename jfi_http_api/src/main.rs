use axum::{routing::get, Router};
use std::{env, net::SocketAddr};
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

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
