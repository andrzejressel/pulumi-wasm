#[derive(serde::Serialize)]
pub struct RateLimitMatchResponse {
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<std::collections::HashMap<String, String>>>>,
    #[serde(rename = "originTraffic")]
    pub r#origin_traffic: Box<Option<bool>>,
    #[serde(rename = "statuses")]
    pub r#statuses: Box<Option<Vec<i32>>>,
}
