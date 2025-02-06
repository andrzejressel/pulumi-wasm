#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementXssMatchStatementFieldToMatchCookies {
    /// The filter to use to identify the subset of cookies to inspect in a web request. You must specify exactly one setting: either `all`, `included_cookies` or `excluded_cookies`. More details: [CookieMatchPattern](https://docs.aws.amazon.com/waf/latest/APIReference/API_CookieMatchPattern.html)
    #[builder(into)]
    #[serde(rename = "matchPatterns")]
    pub r#match_patterns: Box<Vec<super::super::types::wafv2::RuleGroupRuleStatementXssMatchStatementFieldToMatchCookiesMatchPattern>>,
    /// The parts of the cookies to inspect with the rule inspection criteria. If you specify All, AWS WAF inspects both keys and values. Valid values: `ALL`, `KEY`, `VALUE`
    #[builder(into)]
    #[serde(rename = "matchScope")]
    pub r#match_scope: Box<String>,
    /// What AWS WAF should do if the cookies of the request are larger than AWS WAF can inspect. AWS WAF does not support inspecting the entire contents of request cookies when they exceed 8 KB (8192 bytes) or 200 total cookies. The underlying host service forwards a maximum of 200 cookies and at most 8 KB of cookie contents to AWS WAF. Valid values: `CONTINUE`, `MATCH`, `NO_MATCH`
    #[builder(into)]
    #[serde(rename = "oversizeHandling")]
    pub r#oversize_handling: Box<String>,
}
