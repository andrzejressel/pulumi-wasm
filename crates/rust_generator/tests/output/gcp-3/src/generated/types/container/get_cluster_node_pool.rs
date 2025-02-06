#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePool {
    /// Configuration required by cluster autoscaler to adjust the size of the node pool to the current cluster usage.
    #[builder(into)]
    #[serde(rename = "autoscalings")]
    pub r#autoscalings: Box<Vec<super::super::types::container::GetClusterNodePoolAutoscaling>>,
    /// The initial number of nodes for the pool. In regional or multi-zonal clusters, this is the number of nodes per zone. Changing this will force recreation of the resource.
    #[builder(into)]
    #[serde(rename = "initialNodeCount")]
    pub r#initial_node_count: Box<i32>,
    /// The resource URLs of the managed instance groups associated with this node pool.
    #[builder(into)]
    #[serde(rename = "instanceGroupUrls")]
    pub r#instance_group_urls: Box<Vec<String>>,
    /// List of instance group URLs which have been assigned to this node pool.
    #[builder(into)]
    #[serde(rename = "managedInstanceGroupUrls")]
    pub r#managed_instance_group_urls: Box<Vec<String>>,
    /// Node management configuration, wherein auto-repair and auto-upgrade is configured.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Box<Vec<super::super::types::container::GetClusterNodePoolManagement>>,
    /// The maximum number of pods per node in this node pool. Note that this does not work on node pools which are "route-based" - that is, node pools belonging to clusters that do not have IP Aliasing enabled.
    #[builder(into)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Box<i32>,
    /// The name of the cluster.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Creates a unique name for the node pool beginning with the specified prefix. Conflicts with name.
    #[builder(into)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: Box<String>,
    /// Networking configuration for this NodePool. If specified, it overrides the cluster-level defaults.
    #[builder(into)]
    #[serde(rename = "networkConfigs")]
    pub r#network_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNetworkConfig>>,
    /// The configuration of the nodepool
    #[builder(into)]
    #[serde(rename = "nodeConfigs")]
    pub r#node_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfig>>,
    /// The number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling.
    #[builder(into)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<i32>,
    /// The list of zones in which the node pool's nodes should be located. Nodes must be in the region of their regional cluster or in the same region as their cluster's zone for zonal clusters. If unspecified, the cluster-level node_locations will be used.
    #[builder(into)]
    #[serde(rename = "nodeLocations")]
    pub r#node_locations: Box<Vec<String>>,
    /// Specifies the node placement policy
    #[builder(into)]
    #[serde(rename = "placementPolicies")]
    pub r#placement_policies: Box<Vec<super::super::types::container::GetClusterNodePoolPlacementPolicy>>,
    /// Specifies the configuration of queued provisioning
    #[builder(into)]
    #[serde(rename = "queuedProvisionings")]
    pub r#queued_provisionings: Box<Vec<super::super::types::container::GetClusterNodePoolQueuedProvisioning>>,
    /// Specify node upgrade settings to change how many nodes GKE attempts to upgrade at once. The number of nodes upgraded simultaneously is the sum of max_surge and max_unavailable. The maximum number of nodes upgraded simultaneously is limited to 20.
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Box<Vec<super::super::types::container::GetClusterNodePoolUpgradeSetting>>,
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
