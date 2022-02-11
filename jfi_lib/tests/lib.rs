#[tokio::test]
async fn test_get_growth_rate_vec_by_fund_code() {
    let growth_rate_vec = jfi_lib::get_growth_rate_vec_by_fund_code("110026")
        .await
        .unwrap();
    let growth_rate = growth_rate_vec.get(0).unwrap();
    assert_eq!(growth_rate.timestmp, 1316448000000);
    assert_eq!(growth_rate.rate, 0.0);
}

#[tokio::test]
async fn test_get_expect_growth_rate_by_fund_code() {
    let growth_rate_vec = jfi_lib::get_expect_growth_rate_by_fund_code("110026")
        .await
        .unwrap();
    dbg!(growth_rate_vec);
}

#[tokio::test]
async fn test_calculate_jerry_index_by_fund_code() {
    let jerry_index = jfi_lib::calculate_jerry_index_by_fund_code("110026").await;
    dbg!(jerry_index);
}

#[tokio::test]
async fn test_get_baidu_index_by_keyword() {
    let baidu_index = jfi_lib::get_baidu_index_by_keyword("基金").await;
    assert_eq!(
        baidu_index.baidu_date_list.len(),
        baidu_index.baidu_all_index_list.len()
    );
    dbg!(baidu_index);
}
