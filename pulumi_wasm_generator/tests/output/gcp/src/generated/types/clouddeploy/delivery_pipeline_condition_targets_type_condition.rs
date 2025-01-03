#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DeliveryPipelineConditionTargetsTypeCondition {
    /// Human readable error message.
    #[builder(into, default)]
    #[serde(rename = "errorDetails")]
    pub r#error_details: Box<Option<String>>,
    /// True if the targets are all a comparable type. For example this is true if all targets are GKE clusters. This is false if some targets are Cloud Run targets and others are GKE clusters.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<bool>>,
}
