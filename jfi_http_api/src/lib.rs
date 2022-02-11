use axum::{extract::Path, Json};
use serde_json::{json, Value};

pub async fn get_jerry_index_by_fund_code(Path(fund_code): Path<String>) -> Json<Value> {
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

pub async fn get_baidu_index_by_keyword(Path(keyword): Path<String>) -> Json<Value> {
    println!("keyword {}", keyword);
    let baidu_index = jfi_lib::get_baidu_index_by_keyword(&keyword).await;
    println!("baidu_index {:?}", baidu_index);
    Json(json!({
        "success": true,
        "code": 200,
        "message": "ok",
        "data": baidu_index,
    }))
}
