#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatement {
    /// Part of a web request that you want AWS WAF to inspect. See `field_to_match` below for details.
    #[builder(into, default)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<Option<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatch>>,
    /// Sensitivity that you want AWS WAF to use to inspect for SQL injection attacks. Valid values include: `LOW`, `HIGH`.
    #[builder(into, default)]
    #[serde(rename = "sensitivityLevel")]
    pub r#sensitivity_level: Box<Option<String>>,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection. At least one transformation is required. See `text_transformation` below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Box<Vec<super::super::types::wafv2::WebAclRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementTextTransformation>>,
}
