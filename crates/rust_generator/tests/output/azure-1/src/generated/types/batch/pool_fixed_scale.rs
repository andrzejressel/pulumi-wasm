#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PoolFixedScale {
    /// It determines what to do with a node and its running task(s) if the pool size is decreasing. Values are `Requeue`, `RetainedData`, `TaskCompletion` and `Terminate`.
    #[builder(into, default)]
    #[serde(rename = "nodeDeallocationMethod")]
    pub r#node_deallocation_method: Box<Option<String>>,
    /// The timeout for resize operations. Defaults to `PT15M`.
    #[builder(into, default)]
    #[serde(rename = "resizeTimeout")]
    pub r#resize_timeout: Box<Option<String>>,
    /// The number of nodes in the Batch pool. Defaults to `1`.
    #[builder(into, default)]
    #[serde(rename = "targetDedicatedNodes")]
    pub r#target_dedicated_nodes: Box<Option<i32>>,
    /// The number of low priority nodes in the Batch pool. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "targetLowPriorityNodes")]
    pub r#target_low_priority_nodes: Box<Option<i32>>,
}
