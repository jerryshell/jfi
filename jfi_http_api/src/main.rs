use axum::{extract::Path, routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/jerryIndex/fundCode/:fund_code",
            get(get_jerry_index_by_fund_code),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_jerry_index_by_fund_code(Path(fund_code): Path<String>) -> Json<Value> {
    println!("fund_code {}", fund_code);
    let jerry_index = jfi_lib::calculate_jerry_index_by_fund_code(&fund_code).await;
    println!("jerry_index {}", jerry_index);
    Json(json!({
        "data": jerry_index,
    }))
}
