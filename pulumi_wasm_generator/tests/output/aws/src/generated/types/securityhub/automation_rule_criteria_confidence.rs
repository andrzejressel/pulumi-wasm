#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationRuleCriteriaConfidence {
    /// The equal-to condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into, default)]
    #[serde(rename = "eq")]
    pub r#eq: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "gt")]
    pub r#gt: Box<Option<f64>>,
    /// The greater-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into, default)]
    #[serde(rename = "gte")]
    pub r#gte: Box<Option<f64>>,
    #[builder(into, default)]
    #[serde(rename = "lt")]
    pub r#lt: Box<Option<f64>>,
    /// The less-than-equal condition to be applied to a single field when querying for findings, provided as a String.
    #[builder(into, default)]
    #[serde(rename = "lte")]
    pub r#lte: Box<Option<f64>>,
}
