#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodeGroupUpdateConfig {
    /// Desired max number of unavailable worker nodes during node group update.
    #[builder(into, default)]
    #[serde(rename = "maxUnavailable")]
    pub r#max_unavailable: Box<Option<i32>>,
    /// Desired max percentage of unavailable worker nodes during node group update.
    #[builder(into, default)]
    #[serde(rename = "maxUnavailablePercentage")]
    pub r#max_unavailable_percentage: Box<Option<i32>>,
}