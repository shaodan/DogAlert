// Doc: https://docs.open.debank.com/en/

use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub id: String,
    pub cate_id: Option<String>,
    pub chain: String,
    pub time_at: f64,
    // #[serde(rename = "debt_liquidated")]
    // pub debt_liquidated: Value,
    // pub other_addr: String,
    // pub project_id: Option<String>,
    // pub receives: Vec<Recefe>,
    // pub sends: Vec<Send2>,
    // pub token_approve: Option<TokenApprove>,
    // pub tx: Option<Tx>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HistoryData {
    history_list: Vec<History>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HistoryResponse {
    data: HistoryData,
}

// https://api.debank.com/history/list?page_count=100&start_time=0&token_id=&user_addr=0xebcd54d901596ce65fa504d96a397e8463d6262d
pub async fn get_user_history(addr: &str) -> Result<Vec<History>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = "https://api.debank.com/history/list";
    let res = client.get(url).query(&[("user_addr", addr)]).send().await?;
    // eprintln!("Response: {:?} {}", res.version(), res.status());
    // eprintln!("Headers: {:#?}\n", res.headers());
    // let body = res.text().await?;
    // println!("{}", body);
    let hr = res.json::<HistoryResponse>().await?;
    Ok(hr.data.history_list)
}

// https://api.debank.com/portfolio/project_list?user_addr=0xebcd54d901596ce65fa504d96a397e8463d6262d
#[allow(dead_code)]
pub async fn get_user_projects(_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let res = reqwest::get(url).await?;
    Ok(())
}

// https://api.debank.com/token/balance_list?user_addr=0xebcd54d901596ce65fa504d96a397e8463d6262d&is_all=false&chain=mobm
#[allow(dead_code)]
pub async fn get_user_balance(_addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
