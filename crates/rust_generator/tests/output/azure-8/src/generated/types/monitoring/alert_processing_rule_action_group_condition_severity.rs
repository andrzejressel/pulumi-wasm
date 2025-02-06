#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AlertProcessingRuleActionGroupConditionSeverity {
    /// The operator for a given condition. Possible values are `Equals` and `NotEquals`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Specifies list of values to match for a given condition. Possible values are `Sev0`, `Sev1`, `Sev2`, `Sev3`, and `Sev4`.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
