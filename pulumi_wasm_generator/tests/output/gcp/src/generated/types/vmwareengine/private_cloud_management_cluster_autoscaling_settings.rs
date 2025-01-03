#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PrivateCloudManagementClusterAutoscalingSettings {
    /// The map with autoscaling policies applied to the cluster.
    /// The key is the identifier of the policy.
    /// It must meet the following requirements:
    /// * Only contains 1-63 alphanumeric characters and hyphens
    /// * Begins with an alphabetical character
    /// * Ends with a non-hyphen character
    /// * Not formatted as a UUID
    /// * Complies with [RFC 1034](https://datatracker.ietf.org/doc/html/rfc1034) (section 3.5)
    /// Currently the map must contain only one element
    /// that describes the autoscaling policy for compute nodes.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "autoscalingPolicies")]
    pub r#autoscaling_policies: Box<Vec<super::super::types::vmwareengine::PrivateCloudManagementClusterAutoscalingSettingsAutoscalingPolicy>>,
    /// The minimum duration between consecutive autoscale operations.
    /// It starts once addition or removal of nodes is fully completed.
    /// Minimum cool down period is 30m.
    /// Cool down period must be in whole minutes (for example, 30m, 31m, 50m).
    /// Mandatory for successful addition of autoscaling settings in cluster.
    #[builder(into, default)]
    #[serde(rename = "coolDownPeriod")]
    pub r#cool_down_period: Box<Option<String>>,
    /// Maximum number of nodes of any type in a cluster.
    /// Mandatory for successful addition of autoscaling settings in cluster.
    #[builder(into, default)]
    #[serde(rename = "maxClusterNodeCount")]
    pub r#max_cluster_node_count: Box<Option<i32>>,
    /// Minimum number of nodes of any type in a cluster.
    /// Mandatory for successful addition of autoscaling settings in cluster.
    #[builder(into, default)]
    #[serde(rename = "minClusterNodeCount")]
    pub r#min_cluster_node_count: Box<Option<i32>>,
}
