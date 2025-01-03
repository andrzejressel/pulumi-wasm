#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRule {
    /// Action that AWS WAF should take on a web request when it matches the rule's statement. This is used only for rules whose **statements do not reference a rule group**. See `action` for details.
    #[builder(into, default)]
    #[serde(rename = "action")]
    pub r#action: Box<Option<super::super::types::wafv2::WebAclRuleAction>>,
    /// Specifies how AWS WAF should handle CAPTCHA evaluations. See `captcha_config` below for details.
    #[builder(into, default)]
    #[serde(rename = "captchaConfig")]
    pub r#captcha_config: Box<Option<super::super::types::wafv2::WebAclRuleCaptchaConfig>>,
    /// Friendly name of the rule. Note that the provider assumes that rules with names matching this pattern, `^ShieldMitigationRuleGroup_<account-id>_<web-acl-guid>_.*`, are AWS-added for [automatic application layer DDoS mitigation activities](https://docs.aws.amazon.com/waf/latest/developerguide/ddos-automatic-app-layer-response-rg.html). Such rules will be ignored by the provider unless you explicitly include them in your configuration (for example, by using the AWS CLI to discover their properties and creating matching configuration). However, since these rules are owned and managed by AWS, you may get permission errors.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Override action to apply to the rules in a rule group. Used only for rule **statements that reference a rule group**, like `rule_group_reference_statement` and `managed_rule_group_statement`. See `override_action` below for details.
    #[builder(into, default)]
    #[serde(rename = "overrideAction")]
    pub r#override_action: Box<Option<super::super::types::wafv2::WebAclRuleOverrideAction>>,
    /// If you define more than one Rule in a WebACL, AWS WAF evaluates each request against the `rules` in order based on the value of `priority`. AWS WAF processes rules with lower priority first.
    #[builder(into)]
    #[serde(rename = "priority")]
    pub r#priority: Box<i32>,
    /// Labels to apply to web requests that match the rule match statement. See `rule_label` below for details.
    #[builder(into, default)]
    #[serde(rename = "ruleLabels")]
    pub r#rule_labels: Box<Option<Vec<super::super::types::wafv2::WebAclRuleRuleLabel>>>,
    /// The AWS WAF processing statement for the rule, for example `byte_match_statement` or `geo_match_statement`. See `statement` below for details.
    #[builder(into)]
    #[serde(rename = "statement")]
    pub r#statement: Box<super::super::types::wafv2::WebAclRuleStatement>,
    /// Defines and enables Amazon CloudWatch metrics and web request sample collection. See `visibility_config` below for details.
    #[builder(into)]
    #[serde(rename = "visibilityConfig")]
    pub r#visibility_config: Box<super::super::types::wafv2::WebAclRuleVisibilityConfig>,
}
