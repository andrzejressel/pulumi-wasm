#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutomationRuleActionFindingFieldsUpdateSeverity {
    /// The severity value of the finding. The allowed values are the following `INFORMATIONAL`, `LOW`, `MEDIUM`, `HIGH` and `CRITICAL`.
    #[builder(into, default)]
    #[serde(rename = "label")]
    pub r#label: Box<Option<String>>,
    /// The native severity as defined by the AWS service or integrated partner product that generated the finding.
    #[builder(into, default)]
    #[serde(rename = "product")]
    pub r#product: Box<Option<f64>>,
}