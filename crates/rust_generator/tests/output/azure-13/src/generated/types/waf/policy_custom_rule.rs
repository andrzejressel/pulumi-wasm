#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PolicyCustomRule {
    /// Type of action. Possible values are `Allow`, `Block` and `Log`.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Describes if the policy is in enabled state or disabled state. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Specifies what grouping the rate limit will count requests by. Possible values are `GeoLocation`, `ClientAddr` and `None`.
    #[builder(into, default)]
    #[serde(rename = "groupRateLimitBy")]
    pub r#group_rate_limit_by: Box<Option<String>>,
    /// One or more `match_conditions` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "matchConditions")]
    pub r#match_conditions: Box<Vec<super::super::types::waf::PolicyCustomRuleMatchCondition>>,
    /// Gets name of the resource that is unique within a policy. This name can be used to access the resource.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Describes priority of the rule. Rules with a lower value will be evaluated before rules with a higher value.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// Specifies the duration at which the rate limit policy will be applied. Should be used with `RateLimitRule` rule type. Possible values are `FiveMins` and `OneMin`.
    #[builder(into, default)]
    #[serde(rename = "rateLimitDuration")]
    pub r#rate_limit_duration: Box<Option<String>>,
    /// Specifies the threshold value for the rate limit policy. Must be greater than or equal to 1 if provided.
    #[builder(into, default)]
    #[serde(rename = "rateLimitThreshold")]
    pub r#rate_limit_threshold: Box<Option<i32>>,
    /// Describes the type of rule. Possible values are `MatchRule`, `RateLimitRule` and `Invalid`.
    #[builder(into)]
    #[serde(rename = "ruleType")]
    pub r#rule_type: Box<String>,
}
