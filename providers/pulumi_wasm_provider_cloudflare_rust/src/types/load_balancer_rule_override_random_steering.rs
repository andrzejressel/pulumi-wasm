#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideRandomSteering {
    #[serde(rename = "defaultWeight")]
    pub r#default_weight: Box<Option<f64>>,
    #[serde(rename = "poolWeights")]
    pub r#pool_weights: Box<Option<std::collections::HashMap<String, f64>>>,
}
