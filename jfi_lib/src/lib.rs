use anyhow::Result;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GrowthRateResponsePayload {
    x: i64,
    y: f64,
    equity_return: f64,
    unit_money: String,
}

#[derive(Debug)]
struct GrowthRate {
    timestmp: i64,
    nav: f64,
    rate: f64,
}

fn get_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("Chrome/97")
        .build()
        .unwrap()
}

async fn get_growth_rate_vec_by_fund_code(fund_code: &str) -> Result<Vec<GrowthRate>> {
    let client = get_client();
    let url = format!("https://fund.eastmoney.com/pingzhongdata/{}.js", fund_code);
    let response = client.get(url).send().await?.text().await?;
    let json_str = response
        .split("Data_netWorthTrend = ")
        .last()
        .unwrap()
        .split(";/*累计净值走势*/var Data_ACWorthTrend")
        .next()
        .unwrap();
    let reponse_payload_vec =
        serde_json::from_str::<Vec<GrowthRateResponsePayload>>(json_str).unwrap();
    let growth_rate_vec = reponse_payload_vec
        .iter()
        .map(|item| GrowthRate {
            timestmp: item.x,
            nav: item.y,
            rate: item.equity_return,
        })
        .collect::<Vec<GrowthRate>>();
    Ok(growth_rate_vec)
}

async fn get_expect_growth_rate_by_fund_code(fund_code: &str) -> Result<f64> {
    let client = get_client();
    let url = format!("https://fundmobapi.eastmoney.com/FundMNewApi/FundMNFInfo?plat=Android&appType=ttjj&product=EFund&Version=1&deviceid=ssdfsdfsdf&Fcodes={}", fund_code);
    let response = client.get(url).send().await?.text().await?;
    let expect_growth_rate = serde_json::from_str::<Value>(&response)
        .unwrap()
        .get("Datas")
        .unwrap()
        .get(0)
        .unwrap()
        .get("GSZZL")
        .unwrap()
        .as_str()
        .unwrap()
        .parse()
        .unwrap();
    Ok(expect_growth_rate)
}

// 要求 growth_rate_vec 的排序顺序为时间倒序
fn calculate_jerry_index_by_growth_rate_vec(growth_rate_vec: &[f64]) -> f64 {
    let d5_sum = growth_rate_vec.iter().take(5).sum::<f64>();
    let d123_sum = growth_rate_vec.iter().take(123).sum::<f64>();
    let d123_sum_d25_avg = d123_sum / 25f64;
    d5_sum - d123_sum_d25_avg
}

async fn calculate_jerry_index_by_fund_code(fund_code: &str) -> f64 {
    let growth_rate_vec = get_growth_rate_vec_by_fund_code(fund_code).await.unwrap();
    let mut growth_rate_vec = growth_rate_vec
        .iter()
        .map(|item| item.rate)
        .collect::<Vec<f64>>();
    let expect_growth_rate = get_expect_growth_rate_by_fund_code(fund_code)
        .await
        .unwrap();
    growth_rate_vec.push(expect_growth_rate);
    growth_rate_vec.reverse();
    calculate_jerry_index_by_growth_rate_vec(&growth_rate_vec)
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_get_growth_rate_vec_by_fund_code() {
        let growth_rate_vec = crate::get_growth_rate_vec_by_fund_code("110026")
            .await
            .unwrap();
        let growth_rate = growth_rate_vec.get(0).unwrap();
        assert_eq!(growth_rate.timestmp, 1316448000000);
        assert_eq!(growth_rate.nav, 1.0);
        assert_eq!(growth_rate.rate, 0.0);
    }

    #[tokio::test]
    async fn test_get_expect_growth_rate_by_fund_code() {
        let growth_rate_vec = crate::get_expect_growth_rate_by_fund_code("110026")
            .await
            .unwrap();
        dbg!(growth_rate_vec);
    }

    #[tokio::test]
    async fn test_calculate_jerry_index_by_fund_code() {
        let jerry_index = crate::calculate_jerry_index_by_fund_code("110026").await;
        dbg!(jerry_index);
    }
}
