#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NodePoolPlacementPolicy {
    /// If set, refers to the name of a custom resource policy supplied by the user.
    /// The resource policy must be in the same project and region as the node pool.
    /// If not found, InvalidArgument error is returned.
    #[builder(into, default)]
    #[serde(rename = "policyName")]
    pub r#policy_name: Box<Option<String>>,
    /// The [TPU placement topology](https://cloud.google.com/tpu/docs/types-topologies#tpu_topologies) for pod slice node pool.
    #[builder(into, default)]
    #[serde(rename = "tpuTopology")]
    pub r#tpu_topology: Box<Option<String>>,
    /// The type of the policy. Supports a single value: COMPACT.
    /// Specifying COMPACT placement policy type places node pool's nodes in a closer
    /// physical proximity in order to reduce network latency between nodes.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
