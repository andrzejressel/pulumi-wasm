#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRule {
    /// The action that AWS WAF should take on a web request when it matches the rule's statement. Settings at the `aws.wafv2.WebAcl` level can override the rule action setting. See Action below for details.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<super::super::types::wafv2::RuleGroupRuleAction>,
    /// Specifies how AWS WAF should handle CAPTCHA evaluations. See Captcha Configuration below for details.
    #[builder(into, default)]
    #[serde(rename = "captchaConfig")]
    pub r#captcha_config: Box<Option<super::super::types::wafv2::RuleGroupRuleCaptchaConfig>>,
    /// A friendly name of the rule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// If you define more than one Rule in a WebACL, AWS WAF evaluates each request against the `rules` in order based on the value of `priority`. AWS WAF processes rules with lower priority first.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// Labels to apply to web requests that match the rule match statement. See Rule Label below for details.
    #[builder(into, default)]
    #[serde(rename = "ruleLabels")]
    pub r#rule_labels: Box<Option<Vec<super::super::types::wafv2::RuleGroupRuleRuleLabel>>>,
    /// The AWS WAF processing statement for the rule, for example `byte_match_statement` or `geo_match_statement`. See Statement below for details.
    #[builder(into)]
    #[serde(rename = "statement")]
    pub r#statement: Box<super::super::types::wafv2::RuleGroupRuleStatement>,
    /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See Visibility Configuration below for details.
    #[builder(into)]
    #[serde(rename = "visibilityConfig")]
    pub r#visibility_config: Box<super::super::types::wafv2::RuleGroupRuleVisibilityConfig>,
}
