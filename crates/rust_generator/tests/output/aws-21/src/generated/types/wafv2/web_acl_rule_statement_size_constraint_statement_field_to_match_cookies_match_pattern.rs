#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct WebAclRuleStatementSizeConstraintStatementFieldToMatchCookiesMatchPattern {
    /// An empty configuration block that is used for inspecting all headers.
    #[builder(into, default)]
    #[serde(rename = "all")]
    pub r#all: Box<Option<super::super::types::wafv2::WebAclRuleStatementSizeConstraintStatementFieldToMatchCookiesMatchPatternAll>>,
    #[builder(into, default)]
    #[serde(rename = "excludedCookies")]
    pub r#excluded_cookies: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "includedCookies")]
    pub r#included_cookies: Box<Option<Vec<String>>>,
}
