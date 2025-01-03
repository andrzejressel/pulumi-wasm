#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct V2VmAcceleratorConfig {
    /// Topology of TPU in chips.
    #[builder(into)]
    #[serde(rename = "topology")]
    pub r#topology: Box<String>,
    /// Type of TPU. Please select one of the allowed types: https://cloud.google.com/tpu/docs/reference/rest/v2/AcceleratorConfig#Type
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
