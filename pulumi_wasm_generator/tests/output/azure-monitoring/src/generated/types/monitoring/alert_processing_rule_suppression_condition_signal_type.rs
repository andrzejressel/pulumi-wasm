#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertProcessingRuleSuppressionConditionSignalType {
    /// The operator for a given condition. Possible values are `Equals` and `NotEquals`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Specifies a list of values to match for a given condition. Possible values are `Metric`, `Log`, `Unknown`, and `Health`.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
