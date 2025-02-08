#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AiFeatureOnlineStoreBigtableAutoScaling {
    /// A percentage of the cluster's CPU capacity. Can be from 10% to 80%. When a cluster's CPU utilization exceeds the target that you have set, Bigtable immediately adds nodes to the cluster. When CPU utilization is substantially lower than the target, Bigtable removes nodes. If not set will default to 50%.
    #[builder(into, default)]
    #[serde(rename = "cpuUtilizationTarget")]
    pub r#cpu_utilization_target: Box<Option<i32>>,
    /// The maximum number of nodes to scale up to. Must be greater than or equal to minNodeCount, and less than or equal to 10 times of 'minNodeCount'.
    #[builder(into)]
    #[serde(rename = "maxNodeCount")]
    pub r#max_node_count: Box<i32>,
    /// The minimum number of nodes to scale down to. Must be greater than or equal to 1.
    #[builder(into)]
    #[serde(rename = "minNodeCount")]
    pub r#min_node_count: Box<i32>,
}
