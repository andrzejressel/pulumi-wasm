#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationSourceRuleSetRule {
    /// A value that will be compared with the value in `property_path`.
    #[builder(into)]
    #[serde(rename = "expectedValue")]
    pub r#expected_value: Box<String>,
    /// The comparison operator to use, must be one of: `Contains`, `EndsWith`, `Equals`, `GreaterThan`, `GreaterThanOrEqualTo`, `LesserThan`, `LesserThanOrEqualTo`, `NotEquals`, `StartsWith`
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// The JPath of the entity model property that should be checked.
    #[builder(into)]
    #[serde(rename = "propertyPath")]
    pub r#property_path: Box<String>,
    /// The data type of the compared operands, must be one of: `Integer`, `String`, `Boolean` or `Number`.
    /// 
    /// > **NOTE:** The schema for Security Center alerts (when `event_source` is "Alerts") [can be found here](https://docs.microsoft.com/azure/security-center/alerts-schemas?tabs=schema-continuousexport)
    #[builder(into)]
    #[serde(rename = "propertyType")]
    pub r#property_type: Box<String>,
}
