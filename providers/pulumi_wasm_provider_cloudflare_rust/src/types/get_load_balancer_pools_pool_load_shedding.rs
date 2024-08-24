#[derive(serde::Serialize)]
pub struct GetLoadBalancerPoolsPoolLoadShedding {
    /// Percent of traffic to shed 0 - 100.
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`, `random`
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    /// Percent of session traffic to shed 0 - 100.
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}
