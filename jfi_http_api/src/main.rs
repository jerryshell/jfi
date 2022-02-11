use std::{env, net::SocketAddr};

use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/jerryIndex/fundCode/:fund_code",
            get(get_jerry_index_by_fund_code),
        )
        .route(
            "/baiduIndex/keyword/:keyword",
            get(get_baidu_index_by_keyword),
        );

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_jerry_index_by_fund_code(Path(fund_code): Path<String>) -> Json<Value> {
    println!("fund_code {}", fund_code);
    let jerry_index = jfi_lib::calculate_jerry_index_by_fund_code(&fund_code).await;
    println!("jerry_index {}", jerry_index);
    Json(json!({
        "success": true,
        "code": 200,
        "message": "ok",
        "data": jerry_index,
    }))
}

async fn get_baidu_index_by_keyword(Path(keyword): Path<String>) -> Json<Value> {
    println!("keyword {}", keyword);
    Json(json!({
        "success": true,
        "code": 200,
        "message": "ok",
        "data": keyword,
    }))
}
