#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolLoadShedding {
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}
