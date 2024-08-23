#[derive(serde::Serialize)]
pub struct LoadBalancerRuleOverrideLocationStrategy {
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Box<Option<String>>,
}
