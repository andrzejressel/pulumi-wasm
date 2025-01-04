#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPoolTaskSchedulingPolicy {
    /// Supported values are `Pack` and `Spread`. `Pack` means as many tasks as possible (taskSlotsPerNode) should be assigned to each node in the pool before any tasks are assigned to the next node in the pool. `Spread` means that tasks should be assigned evenly across all nodes in the pool.
    #[builder(into)]
    #[serde(rename = "nodeFillType")]
    pub r#node_fill_type: Box<String>,
}
