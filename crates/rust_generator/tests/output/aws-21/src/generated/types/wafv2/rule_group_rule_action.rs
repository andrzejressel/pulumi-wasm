#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RuleGroupRuleAction {
    /// Instructs AWS WAF to allow the web request. See Allow below for details.
    #[builder(into, default)]
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<super::super::types::wafv2::RuleGroupRuleActionAllow>>,
    /// Instructs AWS WAF to block the web request. See Block below for details.
    #[builder(into, default)]
    #[serde(rename = "block")]
    pub r#block: Box<Option<super::super::types::wafv2::RuleGroupRuleActionBlock>>,
    /// Instructs AWS WAF to run a `CAPTCHA` check against the web request. See Captcha below for details.
    #[builder(into, default)]
    #[serde(rename = "captcha")]
    pub r#captcha: Box<Option<super::super::types::wafv2::RuleGroupRuleActionCaptcha>>,
    /// Instructs AWS WAF to run a check against the request to verify that the request is coming from a legitimate client session. See Challenge below for details.
    #[builder(into, default)]
    #[serde(rename = "challenge")]
    pub r#challenge: Box<Option<super::super::types::wafv2::RuleGroupRuleActionChallenge>>,
    /// Instructs AWS WAF to count the web request and allow it. See Count below for details.
    #[builder(into, default)]
    #[serde(rename = "count")]
    pub r#count: Box<Option<super::super::types::wafv2::RuleGroupRuleActionCount>>,
}
