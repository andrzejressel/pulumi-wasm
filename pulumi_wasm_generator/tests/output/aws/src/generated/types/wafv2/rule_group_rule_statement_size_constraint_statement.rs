#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementSizeConstraintStatement {
    /// The operator to use to compare the request part to the size setting. Valid values include: `EQ`, `NE`, `LE`, `LT`, `GE`, or `GT`.
    #[builder(into)]
    #[serde(rename = "comparisonOperator")]
    pub r#comparison_operator: Box<String>,
    /// The part of a web request that you want AWS WAF to inspect. See Field to Match below for details.
    #[builder(into, default)]
    #[serde(rename = "fieldToMatch")]
    pub r#field_to_match: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementSizeConstraintStatementFieldToMatch>>,
    /// The size, in bytes, to compare to the request part, after any transformations. Valid values are integers between 0 and 21474836480, inclusive.
    #[builder(into)]
    #[serde(rename = "size")]
    pub r#size: Box<i32>,
    /// Text transformations eliminate some of the unusual formatting that attackers use in web requests in an effort to bypass detection.
    /// At least one required.
    /// See Text Transformation below for details.
    #[builder(into)]
    #[serde(rename = "textTransformations")]
    pub r#text_transformations: Box<Vec<super::super::types::wafv2::RuleGroupRuleStatementSizeConstraintStatementTextTransformation>>,
}