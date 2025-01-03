#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceAutoscalingConfigAsymmetricAutoscalingOptionOverridesAutoscalingLimits {
    /// The maximum number of nodes for this specific replica.
    #[builder(into)]
    #[serde(rename = "maxNodes")]
    pub r#max_nodes: Box<i32>,
    /// The minimum number of nodes for this specific replica.
    #[builder(into)]
    #[serde(rename = "minNodes")]
    pub r#min_nodes: Box<i32>,
}
