#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideCountryPool {
    #[serde(rename = "country")]
    pub r#country: Box<String>,
    #[serde(rename = "poolIds")]
    pub r#pool_ids: Box<Vec<String>>,
}
