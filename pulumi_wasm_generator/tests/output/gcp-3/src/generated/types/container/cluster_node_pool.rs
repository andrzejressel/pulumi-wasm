#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodePool {
    /// Configuration required by cluster autoscaler to adjust the size of the node pool to the current cluster usage.
    #[builder(into, default)]
    #[serde(rename = "autoscaling")]
    pub r#autoscaling: Box<Option<super::super::types::container::ClusterNodePoolAutoscaling>>,
    /// The number of nodes to create in this
    /// cluster's default node pool. In regional or multi-zonal clusters, this is the
    /// number of nodes per zone. Must be set if `node_pool` is not set. If you're using
    /// `gcp.container.NodePool` objects with no default node pool, you'll need to
    /// set this to a value of at least `1`, alongside setting
    /// `remove_default_node_pool` to `true`.
    #[builder(into, default)]
    #[serde(rename = "initialNodeCount")]
    pub r#initial_node_count: Box<Option<i32>>,
    /// The resource URLs of the managed instance groups associated with this node pool.
    #[builder(into, default)]
    #[serde(rename = "instanceGroupUrls")]
    pub r#instance_group_urls: Box<Option<Vec<String>>>,
    /// List of instance group URLs which have been assigned to this node pool.
    #[builder(into, default)]
    #[serde(rename = "managedInstanceGroupUrls")]
    pub r#managed_instance_group_urls: Box<Option<Vec<String>>>,
    /// Node management configuration, wherein auto-repair and auto-upgrade is configured.
    #[builder(into, default)]
    #[serde(rename = "management")]
    pub r#management: Box<Option<super::super::types::container::ClusterNodePoolManagement>>,
    /// The maximum number of pods per node in this node pool. Note that this does not work on node pools which are "route-based" - that is, node pools belonging to clusters that do not have IP Aliasing enabled.
    #[builder(into, default)]
    #[serde(rename = "maxPodsPerNode")]
    pub r#max_pods_per_node: Box<Option<i32>>,
    /// The name of the cluster, unique within the project and
    /// location.
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Creates a unique name for the node pool beginning with the specified prefix. Conflicts with name.
    #[builder(into, default)]
    #[serde(rename = "namePrefix")]
    pub r#name_prefix: Box<Option<String>>,
    /// Configuration for
    /// [Adding Pod IP address ranges](https://cloud.google.com/kubernetes-engine/docs/how-to/multi-pod-cidr)) to the node pool. Structure is documented below
    #[builder(into, default)]
    #[serde(rename = "networkConfig")]
    pub r#network_config: Box<Option<super::super::types::container::ClusterNodePoolNetworkConfig>>,
    /// Parameters used in creating the default node pool.
    /// Generally, this field should not be used at the same time as a
    /// `gcp.container.NodePool` or a `node_pool` block; this configuration
    /// manages the default node pool, which isn't recommended to be used.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "nodeConfig")]
    pub r#node_config: Box<Option<super::super::types::container::ClusterNodePoolNodeConfig>>,
    /// The number of nodes per instance group. This field can be used to update the number of nodes per instance group but should not be used alongside autoscaling.
    #[builder(into, default)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<Option<i32>>,
    /// The list of zones in which the cluster's nodes
    /// are located. Nodes must be in the region of their regional cluster or in the
    /// same region as their cluster's zone for zonal clusters. If this is specified for
    /// a zonal cluster, omit the cluster's zone.
    /// 
    /// > A "multi-zonal" cluster is a zonal cluster with at least one additional zone
    /// defined; in a multi-zonal cluster, the cluster master is only present in a
    /// single zone while nodes are present in each of the primary zone and the node
    /// locations. In contrast, in a regional cluster, cluster master nodes are present
    /// in multiple zones in the region. For that reason, regional clusters should be
    /// preferred.
    #[builder(into, default)]
    #[serde(rename = "nodeLocations")]
    pub r#node_locations: Box<Option<Vec<String>>>,
    /// Specifies the node placement policy
    #[builder(into, default)]
    #[serde(rename = "placementPolicy")]
    pub r#placement_policy: Box<Option<super::super::types::container::ClusterNodePoolPlacementPolicy>>,
    /// Specifies the configuration of queued provisioning
    #[builder(into, default)]
    #[serde(rename = "queuedProvisioning")]
    pub r#queued_provisioning: Box<Option<super::super::types::container::ClusterNodePoolQueuedProvisioning>>,
    /// Specify node upgrade settings to change how many nodes GKE attempts to upgrade at once. The number of nodes upgraded simultaneously is the sum of max_surge and max_unavailable. The maximum number of nodes upgraded simultaneously is limited to 20.
    #[builder(into, default)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Box<Option<super::super::types::container::ClusterNodePoolUpgradeSettings>>,
    #[builder(into, default)]
    #[serde(rename = "version")]
    pub r#version: Box<Option<String>>,
}
