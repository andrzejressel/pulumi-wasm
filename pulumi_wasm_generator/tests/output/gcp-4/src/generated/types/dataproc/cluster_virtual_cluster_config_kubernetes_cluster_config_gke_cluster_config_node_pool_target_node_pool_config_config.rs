#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterVirtualClusterConfigKubernetesClusterConfigGkeClusterConfigNodePoolTargetNodePoolConfigConfig {
    /// The number of local SSD disks to attach to the node, 
    /// which is limited by the maximum number of disks allowable per zone.
    #[builder(into, default)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: Box<Option<i32>>,
    /// The name of a Compute Engine machine type.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// Minimum CPU platform to be used by this instance. 
    /// The instance may be scheduled on the specified or a newer CPU platform.
    /// Specify the friendly names of CPU platforms, such as "Intel Haswell" or "Intel Sandy Bridge".
    #[builder(into, default)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<Option<String>>,
    /// Whether the nodes are created as preemptible VM instances. 
    /// Preemptible nodes cannot be used in a node pool with the CONTROLLER role or in the DEFAULT node pool if the
    /// CONTROLLER role is not assigned (the DEFAULT node pool will assume the CONTROLLER role).
    #[builder(into, default)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<Option<bool>>,
    /// Spot flag for enabling Spot VM, which is a rebrand of the existing preemptible flag.
    #[builder(into, default)]
    #[serde(rename = "spot")]
    pub r#spot: Box<Option<bool>>,
}
