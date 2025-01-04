#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementXssMatchStatementFieldToMatchHeaderMatchPattern {
    /// An empty configuration block that is used for inspecting all headers.
    #[builder(into, default)]
    #[serde(rename = "all")]
    pub r#all: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementXssMatchStatementFieldToMatchHeaderMatchPatternAll>>,
    /// An array of strings that will be used for inspecting headers that do not have a key that matches one of the provided values.
    #[builder(into, default)]
    #[serde(rename = "excludedHeaders")]
    pub r#excluded_headers: Box<Option<Vec<String>>>,
    /// An array of strings that will be used for inspecting headers that have a key that matches one of the provided values.
    #[builder(into, default)]
    #[serde(rename = "includedHeaders")]
    pub r#included_headers: Box<Option<Vec<String>>>,
}
