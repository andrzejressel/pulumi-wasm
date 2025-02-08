#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetSecurityPolicyRule {
    /// Action to take when match matches the request.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// An optional description of this rule. Max size is 64.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Additional actions that are performed on headers.
    #[builder(into)]
    #[serde(rename = "headerActions")]
    pub r#header_actions: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleHeaderAction>>,
    /// A match condition that incoming traffic is evaluated against. If it evaluates to true, the corresponding action is enforced.
    #[builder(into)]
    #[serde(rename = "matches")]
    pub r#matches: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleMatch>>,
    /// Preconfigured WAF configuration to be applied for the rule. If the rule does not evaluate preconfigured WAF rules, i.e., if evaluatePreconfiguredWaf() is not used, this field will have no effect.
    #[builder(into)]
    #[serde(rename = "preconfiguredWafConfigs")]
    pub r#preconfigured_waf_configs: Box<Vec<super::super::types::compute::GetSecurityPolicyRulePreconfiguredWafConfig>>,
    /// When set to true, the action specified above is not enforced. Stackdriver logs for requests that trigger a preview action are annotated as such.
    #[builder(into)]
    #[serde(rename = "preview")]
    pub r#preview: Box<bool>,
    /// An unique positive integer indicating the priority of evaluation for a rule. Rules are evaluated from highest priority (lowest numerically) to lowest priority (highest numerically) in order.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// Rate limit threshold for this security policy. Must be specified if the action is "rate_based_ban" or "throttle". Cannot be specified for any other actions.
    #[builder(into)]
    #[serde(rename = "rateLimitOptions")]
    pub r#rate_limit_options: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleRateLimitOption>>,
    /// Parameters defining the redirect action. Cannot be specified for any other actions.
    #[builder(into)]
    #[serde(rename = "redirectOptions")]
    pub r#redirect_options: Box<Vec<super::super::types::compute::GetSecurityPolicyRuleRedirectOption>>,
}
