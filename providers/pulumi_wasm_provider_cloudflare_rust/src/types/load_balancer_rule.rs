#[derive(serde::Serialize)]
pub struct LoadBalancerRule {
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<crate::types::LoadBalancerRuleFixedResponse>>,
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<crate::types::LoadBalancerRuleOverride>>>,
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    #[serde(rename = "terminates")]
    pub r#terminates: Box<Option<bool>>,
}
