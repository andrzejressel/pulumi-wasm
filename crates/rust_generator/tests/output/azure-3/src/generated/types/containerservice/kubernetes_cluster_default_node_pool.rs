#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct KubernetesClusterDefaultNodePool {
    /// Should [the Kubernetes Auto Scaler](https://docs.microsoft.com/azure/aks/cluster-autoscaler) be enabled for this Node Pool?
    /// 
    /// > **Note:** This requires that the `type` is set to `VirtualMachineScaleSets`.
    /// 
    /// > **Note:** If you're using AutoScaling, you may wish to use [`ignoreChanges` functionality](https://www.pulumi.com/docs/intro/concepts/programming-model/#ignorechanges) to ignore changes to the `node_count` field.
    #[builder(into, default)]
    #[serde(rename = "autoScalingEnabled")]
    pub r#auto_scaling_enabled: Box<Option<bool>>,
    /// Specifies the ID of the Capacity Reservation Group within which this AKS Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "capacityReservationGroupId")]
    pub r#capacity_reservation_group_id: Box<Option<String>>,
    /// Should the nodes in this Node Pool have Federal Information Processing Standard enabled? `temporary_name_for_rotation` must be specified when changing this block. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "fipsEnabled")]
    pub r#fips_enabled: Box<Option<bool>>,
    /// Specifies the GPU MIG instance profile for supported GPU VM SKU. The allowed values are `MIG1g`, `MIG2g`, `MIG3g`, `MIG4g` and `MIG7g`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "gpuInstance")]
    pub r#gpu_instance: Box<Option<String>>,
    /// Should the nodes in the Default Node Pool have host encryption enabled? `temporary_name_for_rotation` must be specified when changing this property.
    /// 
    /// > **Note:** This requires that the  Feature `Microsoft.ContainerService/EnableEncryptionAtHost` is enabled and the Resource Provider is registered.
    #[builder(into, default)]
    #[serde(rename = "hostEncryptionEnabled")]
    pub r#host_encryption_enabled: Box<Option<bool>>,
    /// Specifies the ID of the Host Group within which this AKS Cluster should be created. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "hostGroupId")]
    pub r#host_group_id: Box<Option<String>>,
    /// A `kubelet_config` block as defined below. `temporary_name_for_rotation` must be specified when changing this block.
    #[builder(into, default)]
    #[serde(rename = "kubeletConfig")]
    pub r#kubelet_config: Box<Option<super::super::types::containerservice::KubernetesClusterDefaultNodePoolKubeletConfig>>,
    /// The type of disk used by kubelet. Possible values are `OS` and `Temporary`.
    #[builder(into, default)]
    #[serde(rename = "kubeletDiskType")]
    pub r#kubelet_disk_type: Box<Option<String>>,
    /// A `linux_os_config` block as defined below. `temporary_name_for_rotation` must be specified when changing this block.
    #[builder(into, default)]
    #[serde(rename = "linuxOsConfig")]
    pub r#linux_os_config: Box<Option<super::super::types::containerservice::KubernetesClusterDefaultNodePoolLinuxOsConfig>>,
    #[builder(into, default)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Box<Option<i32>>,
    /// The maximum number of pods that can run on each agent. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into, default)]
    #[serde(rename = "maxPods")]
    pub r#max_pods: Box<Option<i32>>,
    #[builder(into, default)]
    #[serde(rename = "minCount")]
    pub r#min_count: Box<Option<i32>>,
    /// The name which should be used for the default Kubernetes Node Pool.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "nodeCount")]
    pub r#node_count: Box<Option<i32>>,
    /// A map of Kubernetes labels which should be applied to nodes in the Default Node Pool.
    #[builder(into, default)]
    #[serde(rename = "nodeLabels")]
    pub r#node_labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// A `node_network_profile` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "nodeNetworkProfile")]
    pub r#node_network_profile: Box<Option<super::super::types::containerservice::KubernetesClusterDefaultNodePoolNodeNetworkProfile>>,
    /// Should nodes in this Node Pool have a Public IP Address? `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into, default)]
    #[serde(rename = "nodePublicIpEnabled")]
    pub r#node_public_ip_enabled: Box<Option<bool>>,
    /// Resource ID for the Public IP Addresses Prefix for the nodes in this Node Pool. `node_public_ip_enabled` should be `true`. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "nodePublicIpPrefixId")]
    pub r#node_public_ip_prefix_id: Box<Option<String>>,
    /// Enabling this option will taint default node pool with `CriticalAddonsOnly=true:NoSchedule` taint. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into, default)]
    #[serde(rename = "onlyCriticalAddonsEnabled")]
    pub r#only_critical_addons_enabled: Box<Option<bool>>,
    /// Version of Kubernetes used for the Agents. If not specified, the default node pool will be created with the version specified by `kubernetes_version`. If both are unspecified, the latest recommended version will be used at provisioning time (but won't auto-upgrade). AKS does not require an exact patch version to be specified, minor version aliases such as `1.22` are also supported. - The minor version's latest GA patch is automatically chosen in that case. More details can be found in [the documentation](https://docs.microsoft.com/en-us/azure/aks/supported-kubernetes-versions?tabs=azure-cli#alias-minor-version).
    /// 
    /// > **Note:** This version must be supported by the Kubernetes Cluster - as such the version of Kubernetes used on the Cluster/Control Plane may need to be upgraded first.
    #[builder(into, default)]
    #[serde(rename = "orchestratorVersion")]
    pub r#orchestrator_version: Box<Option<String>>,
    /// The size of the OS Disk which should be used for each agent in the Node Pool. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into, default)]
    #[serde(rename = "osDiskSizeGb")]
    pub r#os_disk_size_gb: Box<Option<i32>>,
    /// The type of disk which should be used for the Operating System. Possible values are `Ephemeral` and `Managed`. Defaults to `Managed`. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into, default)]
    #[serde(rename = "osDiskType")]
    pub r#os_disk_type: Box<Option<String>>,
    /// Specifies the OS SKU used by the agent pool. Possible values are `AzureLinux`, `Ubuntu`, `Windows2019` and `Windows2022`. If not specified, the default is `Ubuntu` if OSType=Linux or `Windows2019` if OSType=Windows. And the default Windows OSSKU will be changed to `Windows2022` after Windows2019 is deprecated. Changing this from `AzureLinux` or `Ubuntu` to `AzureLinux` or `Ubuntu` will not replace the resource, otherwise `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into, default)]
    #[serde(rename = "osSku")]
    pub r#os_sku: Box<Option<String>>,
    /// The ID of the Subnet where the pods in the default Node Pool should exist.
    #[builder(into, default)]
    #[serde(rename = "podSubnetId")]
    pub r#pod_subnet_id: Box<Option<String>>,
    /// The ID of the Proximity Placement Group. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "proximityPlacementGroupId")]
    pub r#proximity_placement_group_id: Box<Option<String>>,
    /// Specifies the autoscaling behaviour of the Kubernetes Cluster. Allowed values are `Delete` and `Deallocate`. Defaults to `Delete`.
    #[builder(into, default)]
    #[serde(rename = "scaleDownMode")]
    pub r#scale_down_mode: Box<Option<String>>,
    /// The ID of the Snapshot which should be used to create this default Node Pool. `temporary_name_for_rotation` must be specified when changing this property.
    #[builder(into, default)]
    #[serde(rename = "snapshotId")]
    pub r#snapshot_id: Box<Option<String>>,
    /// A mapping of tags to assign to the Node Pool.
    /// 
    /// > At this time there's a bug in the AKS API where Tags for a Node Pool are not stored in the correct case - you may wish to use `ignore_changes` functionality to ignore changes to the casing until this is fixed in the AKS API.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Specifies the name of the temporary node pool used to cycle the default node pool for VM resizing.
    #[builder(into, default)]
    #[serde(rename = "temporaryNameForRotation")]
    pub r#temporary_name_for_rotation: Box<Option<String>>,
    /// The type of Node Pool which should be created. Possible values are `VirtualMachineScaleSets`. Defaults to `VirtualMachineScaleSets`. Changing this forces a new resource to be created.
    /// 
    /// > **Note:** When creating a cluster that supports multiple node pools, the cluster must use `VirtualMachineScaleSets`. For more information on the limitations of clusters using multiple node pools see [the documentation](https://learn.microsoft.com/en-us/azure/aks/use-multiple-node-pools#limitations).
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// Used to specify whether the UltraSSD is enabled in the Default Node Pool. Defaults to `false`. See [the documentation](https://docs.microsoft.com/azure/aks/use-ultra-disks) for more information. `temporary_name_for_rotation` must be specified when attempting a change.
    #[builder(into, default)]
    #[serde(rename = "ultraSsdEnabled")]
    pub r#ultra_ssd_enabled: Box<Option<bool>>,
    /// A `upgrade_settings` block as documented below.
    #[builder(into, default)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Box<Option<super::super::types::containerservice::KubernetesClusterDefaultNodePoolUpgradeSettings>>,
    /// The size of the Virtual Machine, such as `Standard_DS2_v2`. `temporary_name_for_rotation` must be specified when attempting a resize.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Box<String>,
    /// The ID of a Subnet where the Kubernetes Node Pool should exist.
    /// 
    /// > **Note:** A Route Table must be configured on this Subnet.
    #[builder(into, default)]
    #[serde(rename = "vnetSubnetId")]
    pub r#vnet_subnet_id: Box<Option<String>>,
    /// Specifies the workload runtime used by the node pool. Possible value is `OCIContainer`.
    #[builder(into, default)]
    #[serde(rename = "workloadRuntime")]
    pub r#workload_runtime: Box<Option<String>>,
    /// Specifies a list of Availability Zones in which this Kubernetes Cluster should be located. `temporary_name_for_rotation` must be specified when changing this property.
    /// 
    /// > **Note:** This requires that the `type` is set to `VirtualMachineScaleSets` and that `load_balancer_sku` is set to `standard`.
    #[builder(into, default)]
    #[serde(rename = "zones")]
    pub r#zones: Box<Option<Vec<String>>>,
}
