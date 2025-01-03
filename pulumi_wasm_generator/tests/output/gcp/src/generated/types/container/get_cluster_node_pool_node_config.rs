#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterNodePoolNodeConfig {
    /// Specifies options for controlling advanced machine features.
    #[builder(into)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigAdvancedMachineFeature>>,
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool.
    #[builder(into)]
    #[serde(rename = "bootDiskKmsKey")]
    pub r#boot_disk_kms_key: Box<String>,
    /// Configuration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after pool creation without deleting and recreating the entire pool.
    #[builder(into)]
    #[serde(rename = "confidentialNodes")]
    pub r#confidential_nodes: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigConfidentialNode>>,
    /// Parameters for containerd configuration.
    #[builder(into)]
    #[serde(rename = "containerdConfigs")]
    pub r#containerd_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigContainerdConfig>>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB.
    #[builder(into)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<i32>,
    /// Type of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<String>,
    /// List of kubernetes taints applied to each node.
    #[builder(into)]
    #[serde(rename = "effectiveTaints")]
    pub r#effective_taints: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigEffectiveTaint>>,
    /// If enabled boot disks are configured with confidential mode.
    #[builder(into)]
    #[serde(rename = "enableConfidentialStorage")]
    pub r#enable_confidential_storage: Box<bool>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into)]
    #[serde(rename = "ephemeralStorageConfigs")]
    pub r#ephemeral_storage_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigEphemeralStorageConfig>>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into)]
    #[serde(rename = "ephemeralStorageLocalSsdConfigs")]
    pub r#ephemeral_storage_local_ssd_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigEphemeralStorageLocalSsdConfig>>,
    /// Enable or disable NCCL Fast Socket in the node pool.
    #[builder(into)]
    #[serde(rename = "fastSockets")]
    pub r#fast_sockets: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigFastSocket>>,
    /// GCFS configuration for this node.
    #[builder(into)]
    #[serde(rename = "gcfsConfigs")]
    pub r#gcfs_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigGcfsConfig>>,
    /// List of the type and count of accelerator cards attached to the instance.
    #[builder(into)]
    #[serde(rename = "guestAccelerators")]
    pub r#guest_accelerators: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigGuestAccelerator>>,
    /// Enable or disable gvnic in the node pool.
    #[builder(into)]
    #[serde(rename = "gvnics")]
    pub r#gvnics: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigGvnic>>,
    /// The maintenance policy for the hosts on which the GKE VMs run on.
    #[builder(into)]
    #[serde(rename = "hostMaintenancePolicies")]
    pub r#host_maintenance_policies: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigHostMaintenancePolicy>>,
    /// The image type to use for this node. Note that for a given image type, the latest version of it will be used.
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<String>,
    /// Node kubelet configs.
    #[builder(into)]
    #[serde(rename = "kubeletConfigs")]
    pub r#kubelet_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigKubeletConfig>>,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node.
    #[builder(into)]
    #[serde(rename = "labels")]
    pub r#labels: Box<std::collections::HashMap<String, String>>,
    /// Parameters that can be configured on Linux nodes.
    #[builder(into)]
    #[serde(rename = "linuxNodeConfigs")]
    pub r#linux_node_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigLinuxNodeConfig>>,
    /// Parameters for raw-block local NVMe SSDs.
    #[builder(into)]
    #[serde(rename = "localNvmeSsdBlockConfigs")]
    pub r#local_nvme_ssd_block_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigLocalNvmeSsdBlockConfig>>,
    /// The number of local SSD disks to be attached to the node.
    #[builder(into)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: Box<i32>,
    /// LocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node.
    #[builder(into)]
    #[serde(rename = "localSsdEncryptionMode")]
    pub r#local_ssd_encryption_mode: Box<String>,
    /// Type of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT.
    #[builder(into)]
    #[serde(rename = "loggingVariant")]
    pub r#logging_variant: Box<String>,
    /// The name of a Google Compute Engine machine type.
    #[builder(into)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<String>,
    /// The metadata key/value pairs assigned to instances in the cluster.
    #[builder(into)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<std::collections::HashMap<String, String>>,
    /// Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform.
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<String>,
    /// Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes.
    #[builder(into)]
    #[serde(rename = "nodeGroup")]
    pub r#node_group: Box<String>,
    /// The set of Google API scopes to be made available on all of the node VMs.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Vec<String>>,
    /// Whether the nodes are created as preemptible VM instances.
    #[builder(into)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<bool>,
    /// The reservation affinity configuration for the node pool.
    #[builder(into)]
    #[serde(rename = "reservationAffinities")]
    pub r#reservation_affinities: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigReservationAffinity>>,
    /// The GCE resource labels (a map of key/value pairs) to be applied to the node pool.
    #[builder(into)]
    #[serde(rename = "resourceLabels")]
    pub r#resource_labels: Box<std::collections::HashMap<String, String>>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<std::collections::HashMap<String, String>>,
    /// Sandbox configuration for this node.
    #[builder(into)]
    #[serde(rename = "sandboxConfigs")]
    pub r#sandbox_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigSandboxConfig>>,
    /// Secondary boot disks for preloading data or container images.
    #[builder(into)]
    #[serde(rename = "secondaryBootDisks")]
    pub r#secondary_boot_disks: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigSecondaryBootDisk>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<String>,
    /// Shielded Instance options.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfigs")]
    pub r#shielded_instance_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigShieldedInstanceConfig>>,
    /// Node affinity options for sole tenant node pools.
    #[builder(into)]
    #[serde(rename = "soleTenantConfigs")]
    pub r#sole_tenant_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigSoleTenantConfig>>,
    /// Whether the nodes are created as spot VM instances.
    #[builder(into)]
    #[serde(rename = "spot")]
    pub r#spot: Box<bool>,
    /// The list of Storage Pools where boot disks are provisioned.
    #[builder(into)]
    #[serde(rename = "storagePools")]
    pub r#storage_pools: Box<Vec<String>>,
    /// The list of instance tags applied to all nodes.
    #[builder(into)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Vec<String>>,
    /// List of Kubernetes taints to be applied to each node.
    #[builder(into)]
    #[serde(rename = "taints")]
    pub r#taints: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigTaint>>,
    /// The workload metadata configuration for this node.
    #[builder(into)]
    #[serde(rename = "workloadMetadataConfigs")]
    pub r#workload_metadata_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigWorkloadMetadataConfig>>,
}
