#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatascanDataQualitySpecRuleTableConditionExpectation {
    /// The SQL expression.
    #[builder(into)]
    #[serde(rename = "sqlExpression")]
    pub r#sql_expression: Box<String>,
}
