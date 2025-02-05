#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetKubernetesClusterAgentPoolProfile {
    /// If the auto-scaler is enabled.
    #[builder(into)]
    #[serde(rename = "autoScalingEnabled")]
    pub r#auto_scaling_enabled: Box<bool>,
    /// The number of Agents (VMs) in the Pool.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// Maximum number of nodes for auto-scaling
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Box<i32>,
    /// The maximum number of pods that can run on each agent.
    #[builder(into)]
    #[serde(rename = "maxPods")]
    pub r#max_pods: Box<i32>,
    /// Minimum number of nodes for auto-scaling
    #[builder(into)]
    #[serde(rename = "minCount")]
    pub r#min_count: Box<i32>,
    /// The name of the managed Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into)]
    #[serde(rename = "nodeLabels")]
    pub r#node_labels: Box<std::collections::HashMap<String, String>>,
    /// If the Public IPs for the nodes in this Agent Pool are enabled.
    #[builder(into)]
    #[serde(rename = "nodePublicIpEnabled")]
    pub r#node_public_ip_enabled: Box<bool>,
    /// Resource ID for the Public IP Addresses Prefix for the nodes in this Agent Pool.
    #[builder(into)]
    #[serde(rename = "nodePublicIpPrefixId")]
    pub r#node_public_ip_prefix_id: Box<String>,
    #[builder(into)]
    #[serde(rename = "nodeTaints")]
    pub r#node_taints: Box<Vec<String>>,
    /// Kubernetes version used for the Agents.
    #[builder(into)]
    #[serde(rename = "orchestratorVersion")]
    pub r#orchestrator_version: Box<String>,
    /// The size of the Agent VM's Operating System Disk in GB.
    #[builder(into)]
    #[serde(rename = "osDiskSizeGb")]
    pub r#os_disk_size_gb: Box<i32>,
    /// The Operating System used for the Agents.
    #[builder(into)]
    #[serde(rename = "osType")]
    pub r#os_type: Box<String>,
    /// A mapping of tags to assign to the resource.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<std::collections::HashMap<String, String>>,
    /// The type of Managed Service Identity that is configured on this Kubernetes Cluster.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// A `upgrade_settings` block as documented below.
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Box<Vec<super::super::types::containerservice::GetKubernetesClusterAgentPoolProfileUpgradeSetting>>,
    /// The size of each VM in the Agent Pool (e.g. `Standard_F1`).
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Box<String>,
    /// The ID of the Subnet where the Agents in the Pool are provisioned.
    #[builder(into)]
    #[serde(rename = "vnetSubnetId")]
    pub r#vnet_subnet_id: Box<String>,
    /// A list of Availability Zones in which this Kubernetes Cluster is located.
    #[builder(into)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Vec<String>>,
}
