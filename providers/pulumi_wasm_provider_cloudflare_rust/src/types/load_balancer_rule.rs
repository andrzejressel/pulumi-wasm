#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct LoadBalancerRule {
    /// The statement to evaluate to determine if this rule's effects should be applied. An empty condition is always true. See [load balancing rules](https://developers.cloudflare.com/load-balancing/understand-basics/load-balancing-rules).
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// A disabled rule will not be executed.
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// Settings for a HTTP response to return directly to the eyeball if the condition is true. Note: `overrides` or `fixed_response` must be set.
    #[serde(rename = "fixedResponse")]
    pub r#fixed_response: Box<Option<crate::types::LoadBalancerRuleFixedResponse>>,
    /// Human readable name for this rule.
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The load balancer settings to alter if this rule's `condition` is true. Note: `overrides` or `fixed_response` must be set.
    #[serde(rename = "overrides")]
    pub r#overrides: Box<Option<Vec<crate::types::LoadBalancerRuleOverride>>>,
    /// Priority used when determining the order of rule execution. Lower values are executed first. If not provided, the list order will be used.
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
    /// Terminates indicates that if this rule is true no further rules should be executed. Note: setting a `fixed_response` forces this field to `true`.
    #[serde(rename = "terminates")]
    pub r#terminates: Box<Option<bool>>,
}
