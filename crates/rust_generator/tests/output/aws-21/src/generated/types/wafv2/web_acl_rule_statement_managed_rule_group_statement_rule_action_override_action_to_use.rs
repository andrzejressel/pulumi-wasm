#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WebAclRuleStatementManagedRuleGroupStatementRuleActionOverrideActionToUse {
    #[builder(into, default)]
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementRuleActionOverrideActionToUseAllow>>,
    #[builder(into, default)]
    #[serde(rename = "block")]
    pub r#block: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementRuleActionOverrideActionToUseBlock>>,
    /// Instructs AWS WAF to run a Captcha check against the web request. See `captcha` below for details.
    #[builder(into, default)]
    #[serde(rename = "captcha")]
    pub r#captcha: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementRuleActionOverrideActionToUseCaptcha>>,
    /// Instructs AWS WAF to run a check against the request to verify that the request is coming from a legitimate client session. See `challenge` below for details.
    #[builder(into, default)]
    #[serde(rename = "challenge")]
    pub r#challenge: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementRuleActionOverrideActionToUseChallenge>>,
    #[builder(into, default)]
    #[serde(rename = "count")]
    pub r#count: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementRuleActionOverrideActionToUseCount>>,
}
