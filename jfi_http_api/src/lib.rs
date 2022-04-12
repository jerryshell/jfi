use axum::{extract::Path, Json};
use cached::proc_macro::cached;
use serde_json::{json, Value};

#[cached(time = 30)]
async fn calculate_jerry_index_by_fund_code(fund_code: String) -> f64 {
    jfi_lib::calculate_jerry_index_by_fund_code(fund_code).await
}

pub async fn get_jerry_index_by_fund_code(Path(fund_code): Path<String>) -> Json<Value> {
    println!("fund_code {}", fund_code);
    let jerry_index = calculate_jerry_index_by_fund_code(fund_code).await;
    println!("jerry_index {}", jerry_index);
    Json(json!({
        "success": true,
        "code": 200,
        "message": "ok",
        "data": jerry_index,
    }))
}

pub async fn get_baidu_index_by_keyword(Path(keyword): Path<String>) -> Json<Value> {
    println!("keyword {}", keyword);
    let baidu_index = jfi_lib::get_baidu_index_by_keyword(keyword).await.unwrap();
    println!("baidu_index {:?}", baidu_index);
    Json(json!({
        "success": true,
        "code": 200,
        "message": "ok",
        "data": baidu_index,
    }))
}
