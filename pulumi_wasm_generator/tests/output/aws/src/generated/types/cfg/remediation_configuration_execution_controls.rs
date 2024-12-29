#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RemediationConfigurationExecutionControls {
    /// Configuration block for SSM controls. See below.
    #[builder(into, default)]
    #[serde(rename = "ssmControls")]
    pub r#ssm_controls: Box<Option<super::super::types::cfg::RemediationConfigurationExecutionControlsSsmControls>>,
}
