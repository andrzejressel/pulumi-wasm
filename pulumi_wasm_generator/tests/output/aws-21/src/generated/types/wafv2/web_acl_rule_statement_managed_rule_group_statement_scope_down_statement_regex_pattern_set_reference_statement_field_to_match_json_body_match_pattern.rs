#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatementFieldToMatchJsonBodyMatchPattern {
    /// An empty configuration block that is used for inspecting all headers.
    #[builder(into, default)]
    #[serde(rename = "all")]
    pub r#all: Box<Option<super::super::types::wafv2::WebAclRuleStatementManagedRuleGroupStatementScopeDownStatementRegexPatternSetReferenceStatementFieldToMatchJsonBodyMatchPatternAll>>,
    #[builder(into, default)]
    #[serde(rename = "includedPaths")]
    pub r#included_paths: Box<Option<Vec<String>>>,
}
