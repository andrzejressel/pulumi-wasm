#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolFixedScale {
    /// The timeout for resize operations.
    #[builder(into)]
    #[serde(rename = "resizeTimeout")]
    pub r#resize_timeout: Box<String>,
    /// The number of nodes in the Batch pool.
    #[builder(into)]
    #[serde(rename = "targetDedicatedNodes")]
    pub r#target_dedicated_nodes: Box<i32>,
    /// The number of low priority nodes in the Batch pool.
    #[builder(into)]
    #[serde(rename = "targetLowPriorityNodes")]
    pub r#target_low_priority_nodes: Box<i32>,
}
