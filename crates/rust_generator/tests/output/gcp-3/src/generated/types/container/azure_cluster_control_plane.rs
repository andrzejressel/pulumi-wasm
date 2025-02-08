#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AzureClusterControlPlane {
    /// Optional. Configuration related to application-layer secrets encryption.
    #[builder(into, default)]
    #[serde(rename = "databaseEncryption")]
    pub r#database_encryption: Box<Option<super::super::types::container::AzureClusterControlPlaneDatabaseEncryption>>,
    /// Optional. Configuration related to the main volume provisioned for each control plane replica. The main volume is in charge of storing all of the cluster's etcd state. When unspecified, it defaults to a 8-GiB Azure Disk.
    #[builder(into, default)]
    #[serde(rename = "mainVolume")]
    pub r#main_volume: Box<Option<super::super::types::container::AzureClusterControlPlaneMainVolume>>,
    /// Proxy configuration for outbound HTTP(S) traffic.
    #[builder(into, default)]
    #[serde(rename = "proxyConfig")]
    pub r#proxy_config: Box<Option<super::super::types::container::AzureClusterControlPlaneProxyConfig>>,
    /// Configuration for where to place the control plane replicas. Up to three replica placement instances can be specified. If replica_placements is set, the replica placement instances will be applied to the three control plane replicas as evenly as possible.
    #[builder(into, default)]
    #[serde(rename = "replicaPlacements")]
    pub r#replica_placements: Box<Option<Vec<super::super::types::container::AzureClusterControlPlaneReplicaPlacement>>>,
    /// Optional. Configuration related to the root volume provisioned for each control plane replica. When unspecified, it defaults to 32-GiB Azure Disk.
    #[builder(into, default)]
    #[serde(rename = "rootVolume")]
    pub r#root_volume: Box<Option<super::super::types::container::AzureClusterControlPlaneRootVolume>>,
    /// SSH configuration for how to access the underlying control plane machines.
    #[builder(into)]
    #[serde(rename = "sshConfig")]
    pub r#ssh_config: Box<super::super::types::container::AzureClusterControlPlaneSshConfig>,
    /// The ARM ID of the subnet where the control plane VMs are deployed. Example: `/subscriptions//resourceGroups//providers/Microsoft.Network/virtualNetworks//subnets/default`.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// Optional. A set of tags to apply to all underlying control plane Azure resources.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// The Kubernetes version to run on control plane replicas (e.g. `1.19.10-gke.1000`). You can list all supported versions on a given Google Cloud region by calling GetAzureServerConfig.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
    /// Optional. The Azure VM size name. Example: `Standard_DS2_v2`. For available VM sizes, see https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions. When unspecified, it defaults to `Standard_DS2_v2`.
    #[builder(into, default)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Box<Option<String>>,
}
