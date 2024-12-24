#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct GetLoadBalancerPoolsPoolLoadShedding {
    /// Percent of traffic to shed 0 - 100.
    #[builder(into, default)]
    #[serde(rename = "defaultPercent")]
    pub r#default_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`, `random`
    #[builder(into, default)]
    #[serde(rename = "defaultPolicy")]
    pub r#default_policy: Box<Option<String>>,
    /// Percent of session traffic to shed 0 - 100.
    #[builder(into, default)]
    #[serde(rename = "sessionPercent")]
    pub r#session_percent: Box<Option<f64>>,
    /// Method of shedding traffic. Available values: `""`, `hash`
    #[builder(into, default)]
    #[serde(rename = "sessionPolicy")]
    pub r#session_policy: Box<Option<String>>,
}
