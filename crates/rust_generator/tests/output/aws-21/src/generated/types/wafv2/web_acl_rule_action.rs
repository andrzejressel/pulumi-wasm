#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleAction {
    /// Instructs AWS WAF to allow the web request. See `allow` below for details.
    #[builder(into, default)]
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<super::super::types::wafv2::WebAclRuleActionAllow>>,
    /// Instructs AWS WAF to block the web request. See `block` below for details.
    #[builder(into, default)]
    #[serde(rename = "block")]
    pub r#block: Box<Option<super::super::types::wafv2::WebAclRuleActionBlock>>,
    /// Instructs AWS WAF to run a Captcha check against the web request. See `captcha` below for details.
    #[builder(into, default)]
    #[serde(rename = "captcha")]
    pub r#captcha: Box<Option<super::super::types::wafv2::WebAclRuleActionCaptcha>>,
    /// Instructs AWS WAF to run a check against the request to verify that the request is coming from a legitimate client session. See `challenge` below for details.
    #[builder(into, default)]
    #[serde(rename = "challenge")]
    pub r#challenge: Box<Option<super::super::types::wafv2::WebAclRuleActionChallenge>>,
    /// Instructs AWS WAF to count the web request and allow it. See `count` below for details.
    #[builder(into, default)]
    #[serde(rename = "count")]
    pub r#count: Box<Option<super::super::types::wafv2::WebAclRuleActionCount>>,
}
