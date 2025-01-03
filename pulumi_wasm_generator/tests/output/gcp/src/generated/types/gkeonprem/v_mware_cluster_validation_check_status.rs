#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareClusterValidationCheckStatus {
    /// (Output)
    /// Individual checks which failed as part of the Preflight check execution.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "results")]
    pub r#results: Box<Option<Vec<super::super::types::gkeonprem::VMwareClusterValidationCheckStatusResult>>>,
}
