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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaiduIndex {
    pub baidu_date_list: Vec<String>,
    pub baidu_all_index_list: Vec<usize>,
    pub baidu_all_index_list_sum: usize,
    pub baidu_all_index_list_avg: usize,
}

#[derive(Debug)]
struct GrowthRate {
    timestmp: i64,
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
    let mut growth_rate_vec = reponse_payload_vec
        .iter()
        .map(|item| GrowthRate {
            timestmp: item.x,
            rate: item.equity_return,
        })
        .collect::<Vec<GrowthRate>>();
    growth_rate_vec.sort_by_key(|item| item.timestmp);
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
    ((d5_sum - d123_sum_d25_avg) * 1000f64).round() / 1000f64
}

pub async fn calculate_jerry_index_by_fund_code(fund_code: &str) -> f64 {
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

pub async fn get_baidu_index_by_keyword(keyword: &str) -> BaiduIndex {
    let client = get_client();
    let url = format!("https://index.chinaz.com/{}/180", keyword);
    let response = client.get(url).send().await.unwrap().text().await.unwrap();
    let date_str = response
        .split("indexchart.baiduDate = [")
        .last()
        .unwrap()
        .split("];")
        .next()
        .unwrap();
    let date_str_vec = date_str
        .split(',')
        .map(|item| item.replace("\"", ""))
        .collect::<Vec<String>>();
    let index_str = response
        .split("indexchart.baiduAllIndex = [")
        .last()
        .unwrap()
        .split("];")
        .next()
        .unwrap();
    let index_vec = index_str
        .split(',')
        .map(|item| item.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let sum = index_vec.iter().sum::<usize>();
    let avg = sum / index_vec.len();
    BaiduIndex {
        baidu_date_list: date_str_vec,
        baidu_all_index_list: index_vec,
        baidu_all_index_list_sum: sum,
        baidu_all_index_list_avg: avg,
    }
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

    #[tokio::test]
    async fn test_get_baidu_index_by_keyword() {
        let baidu_index = crate::get_baidu_index_by_keyword("基金").await;
        assert_eq!(
            baidu_index.baidu_date_list.len(),
            baidu_index.baidu_all_index_list.len()
        );
        dbg!(baidu_index);
    }
}
