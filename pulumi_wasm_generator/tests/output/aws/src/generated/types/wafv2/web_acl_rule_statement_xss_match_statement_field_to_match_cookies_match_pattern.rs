#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementXssMatchStatementFieldToMatchCookiesMatchPattern {
    /// An empty configuration block that is used for inspecting all headers.
    #[builder(into, default)]
    #[serde(rename = "all")]
    pub r#all: Box<Option<super::super::types::wafv2::WebAclRuleStatementXssMatchStatementFieldToMatchCookiesMatchPatternAll>>,
    #[builder(into, default)]
    #[serde(rename = "excludedCookies")]
    pub r#excluded_cookies: Box<Option<Vec<String>>>,
    #[builder(into, default)]
    #[serde(rename = "includedCookies")]
    pub r#included_cookies: Box<Option<Vec<String>>>,
}