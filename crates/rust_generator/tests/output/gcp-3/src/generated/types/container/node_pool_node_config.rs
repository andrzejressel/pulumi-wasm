#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct NodePoolNodeConfig {
    /// Specifies options for controlling advanced machine features.
    #[builder(into, default)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Box<Option<super::super::types::container::NodePoolNodeConfigAdvancedMachineFeatures>>,
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool.
    #[builder(into, default)]
    #[serde(rename = "bootDiskKmsKey")]
    pub r#boot_disk_kms_key: Box<Option<String>>,
    /// Configuration for the confidential nodes feature, which makes nodes run on confidential VMs. Warning: This configuration can't be changed (or added/removed) after pool creation without deleting and recreating the entire pool.
    #[builder(into, default)]
    #[serde(rename = "confidentialNodes")]
    pub r#confidential_nodes: Box<Option<super::super::types::container::NodePoolNodeConfigConfidentialNodes>>,
    /// Parameters for containerd configuration.
    #[builder(into, default)]
    #[serde(rename = "containerdConfig")]
    pub r#containerd_config: Box<Option<super::super::types::container::NodePoolNodeConfigContainerdConfig>>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// Type of the disk attached to each node. Such as pd-standard, pd-balanced or pd-ssd
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// List of kubernetes taints applied to each node.
    #[builder(into, default)]
    #[serde(rename = "effectiveTaints")]
    pub r#effective_taints: Box<Option<Vec<super::super::types::container::NodePoolNodeConfigEffectiveTaint>>>,
    /// If enabled boot disks are configured with confidential mode.
    #[builder(into, default)]
    #[serde(rename = "enableConfidentialStorage")]
    pub r#enable_confidential_storage: Box<Option<bool>>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into, default)]
    #[serde(rename = "ephemeralStorageConfig")]
    pub r#ephemeral_storage_config: Box<Option<super::super::types::container::NodePoolNodeConfigEphemeralStorageConfig>>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk.
    #[builder(into, default)]
    #[serde(rename = "ephemeralStorageLocalSsdConfig")]
    pub r#ephemeral_storage_local_ssd_config: Box<Option<super::super::types::container::NodePoolNodeConfigEphemeralStorageLocalSsdConfig>>,
    /// Enable or disable NCCL Fast Socket in the node pool.
    #[builder(into, default)]
    #[serde(rename = "fastSocket")]
    pub r#fast_socket: Box<Option<super::super::types::container::NodePoolNodeConfigFastSocket>>,
    /// GCFS configuration for this node.
    #[builder(into, default)]
    #[serde(rename = "gcfsConfig")]
    pub r#gcfs_config: Box<Option<super::super::types::container::NodePoolNodeConfigGcfsConfig>>,
    /// List of the type and count of accelerator cards attached to the instance.
    #[builder(into, default)]
    #[serde(rename = "guestAccelerators")]
    pub r#guest_accelerators: Box<Option<Vec<super::super::types::container::NodePoolNodeConfigGuestAccelerator>>>,
    /// Enable or disable gvnic in the node pool.
    #[builder(into, default)]
    #[serde(rename = "gvnic")]
    pub r#gvnic: Box<Option<super::super::types::container::NodePoolNodeConfigGvnic>>,
    /// The maintenance policy for the hosts on which the GKE VMs run on.
    #[builder(into, default)]
    #[serde(rename = "hostMaintenancePolicy")]
    pub r#host_maintenance_policy: Box<Option<super::super::types::container::NodePoolNodeConfigHostMaintenancePolicy>>,
    /// The image type to use for this node. Note that for a given image type, the latest version of it will be used.
    #[builder(into, default)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<Option<String>>,
    /// Node kubelet configs.
    #[builder(into, default)]
    #[serde(rename = "kubeletConfig")]
    pub r#kubelet_config: Box<Option<super::super::types::container::NodePoolNodeConfigKubeletConfig>>,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node. These will added in addition to any default label(s) that Kubernetes may apply to the node.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Parameters that can be configured on Linux nodes.
    #[builder(into, default)]
    #[serde(rename = "linuxNodeConfig")]
    pub r#linux_node_config: Box<Option<super::super::types::container::NodePoolNodeConfigLinuxNodeConfig>>,
    /// Parameters for raw-block local NVMe SSDs.
    #[builder(into, default)]
    #[serde(rename = "localNvmeSsdBlockConfig")]
    pub r#local_nvme_ssd_block_config: Box<Option<super::super::types::container::NodePoolNodeConfigLocalNvmeSsdBlockConfig>>,
    /// The number of local SSD disks to be attached to the node.
    #[builder(into, default)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: Box<Option<i32>>,
    /// LocalSsdEncryptionMode specified the method used for encrypting the local SSDs attached to the node.
    #[builder(into, default)]
    #[serde(rename = "localSsdEncryptionMode")]
    pub r#local_ssd_encryption_mode: Box<Option<String>>,
    /// Type of logging agent that is used as the default value for node pools in the cluster. Valid values include DEFAULT and MAX_THROUGHPUT.
    #[builder(into, default)]
    #[serde(rename = "loggingVariant")]
    pub r#logging_variant: Box<Option<String>>,
    /// The name of a Google Compute Engine machine type.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// The metadata key/value pairs assigned to instances in the cluster.
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<std::collections::HashMap<String, String>>>,
    /// Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform.
    #[builder(into, default)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<Option<String>>,
    /// Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on sole tenant nodes.
    #[builder(into, default)]
    #[serde(rename = "nodeGroup")]
    pub r#node_group: Box<Option<String>>,
    /// The set of Google API scopes to be made available on all of the node VMs.
    #[builder(into, default)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Option<Vec<String>>>,
    /// Whether the nodes are created as preemptible VM instances.
    #[builder(into, default)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<Option<bool>>,
    /// The configuration of the desired reservation which instances could take capacity from.
    /// Structure is documented below.
    /// 
    /// <a name="nested_autoscaling"></a>The `autoscaling` block supports (either total or per zone limits are required):
    #[builder(into, default)]
    #[serde(rename = "reservationAffinity")]
    pub r#reservation_affinity: Box<Option<super::super::types::container::NodePoolNodeConfigReservationAffinity>>,
    /// The GCE resource labels (a map of key/value pairs) to be applied to the node pool.
    #[builder(into, default)]
    #[serde(rename = "resourceLabels")]
    pub r#resource_labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// A map of resource manager tags. Resource manager tag keys and values have the same definition as resource manager tags. Keys must be in the format tagKeys/{tag_key_id}, and values are in the format tagValues/456. The field is ignored (both PUT & PATCH) when empty.
    #[builder(into, default)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Sandbox configuration for this node.
    #[builder(into, default)]
    #[serde(rename = "sandboxConfig")]
    pub r#sandbox_config: Box<Option<super::super::types::container::NodePoolNodeConfigSandboxConfig>>,
    /// Secondary boot disks for preloading data or container images.
    #[builder(into, default)]
    #[serde(rename = "secondaryBootDisks")]
    pub r#secondary_boot_disks: Box<Option<Vec<super::super::types::container::NodePoolNodeConfigSecondaryBootDisk>>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// Shielded Instance options.
    #[builder(into, default)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Box<Option<super::super::types::container::NodePoolNodeConfigShieldedInstanceConfig>>,
    /// Node affinity options for sole tenant node pools.
    #[builder(into, default)]
    #[serde(rename = "soleTenantConfig")]
    pub r#sole_tenant_config: Box<Option<super::super::types::container::NodePoolNodeConfigSoleTenantConfig>>,
    /// Whether the nodes are created as spot VM instances.
    #[builder(into, default)]
    #[serde(rename = "spot")]
    pub r#spot: Box<Option<bool>>,
    /// The list of Storage Pools where boot disks are provisioned.
    #[builder(into, default)]
    #[serde(rename = "storagePools")]
    pub r#storage_pools: Box<Option<Vec<String>>>,
    /// The list of instance tags applied to all nodes.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// List of Kubernetes taints to be applied to each node.
    #[builder(into, default)]
    #[serde(rename = "taints")]
    pub r#taints: Box<Option<Vec<super::super::types::container::NodePoolNodeConfigTaint>>>,
    /// The workload metadata configuration for this node.
    #[builder(into, default)]
    #[serde(rename = "workloadMetadataConfig")]
    pub r#workload_metadata_config: Box<Option<super::super::types::container::NodePoolNodeConfigWorkloadMetadataConfig>>,
}
