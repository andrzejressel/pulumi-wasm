#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodeConfig {
    /// Specifies options for controlling
    /// advanced machine features. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "advancedMachineFeatures")]
    pub r#advanced_machine_features: Box<Option<super::super::types::container::ClusterNodeConfigAdvancedMachineFeatures>>,
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool. This should be of the form projects/[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME]. For more information about protecting resources with Cloud KMS Keys please see: <https://cloud.google.com/compute/docs/disks/customer-managed-encryption>
    #[builder(into, default)]
    #[serde(rename = "bootDiskKmsKey")]
    pub r#boot_disk_kms_key: Box<Option<String>>,
    /// Configuration for Confidential Nodes feature. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "confidentialNodes")]
    pub r#confidential_nodes: Box<Option<super::super::types::container::ClusterNodeConfigConfidentialNodes>>,
    /// Parameters to customize containerd runtime. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "containerdConfig")]
    pub r#containerd_config: Box<Option<super::super::types::container::ClusterNodeConfigContainerdConfig>>,
    /// Size of the disk attached to each node, specified
    /// in GB. The smallest allowed disk size is 10GB. Defaults to 100GB.
    #[builder(into, default)]
    #[serde(rename = "diskSizeGb")]
    pub r#disk_size_gb: Box<Option<i32>>,
    /// Type of the disk attached to each node
    /// (e.g. 'pd-standard', 'pd-balanced' or 'pd-ssd'). If unspecified, the default disk type is 'pd-standard'
    #[builder(into, default)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<Option<String>>,
    /// List of kubernetes taints applied to each node. Structure is documented above.
    #[builder(into, default)]
    #[serde(rename = "effectiveTaints")]
    pub r#effective_taints: Box<Option<Vec<super::super::types::container::ClusterNodeConfigEffectiveTaint>>>,
    /// Enabling Confidential Storage will create boot disk with confidential mode. It is disabled by default.
    #[builder(into, default)]
    #[serde(rename = "enableConfidentialStorage")]
    pub r#enable_confidential_storage: Box<Option<bool>>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk. Structure is documented below.
    /// 
    #[builder(into, default)]
    #[serde(rename = "ephemeralStorageConfig")]
    pub r#ephemeral_storage_config: Box<Option<super::super::types::container::ClusterNodeConfigEphemeralStorageConfig>>,
    /// Parameters for the ephemeral storage filesystem. If unspecified, ephemeral storage is backed by the boot disk. Structure is documented below.
    /// 
    #[builder(into, default)]
    #[serde(rename = "ephemeralStorageLocalSsdConfig")]
    pub r#ephemeral_storage_local_ssd_config: Box<Option<super::super::types::container::ClusterNodeConfigEphemeralStorageLocalSsdConfig>>,
    /// Parameters for the NCCL Fast Socket feature. If unspecified, NCCL Fast Socket will not be enabled on the node pool.
    /// Node Pool must enable gvnic.
    /// GKE version 1.25.2-gke.1700 or later.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "fastSocket")]
    pub r#fast_socket: Box<Option<super::super::types::container::ClusterNodeConfigFastSocket>>,
    /// Parameters for the Google Container Filesystem (GCFS).
    /// If unspecified, GCFS will not be enabled on the node pool. When enabling this feature you must specify `image_type = "COS_CONTAINERD"` and `node_version` from GKE versions 1.19 or later to use it.
    /// For GKE versions 1.19, 1.20, and 1.21, the recommended minimum `node_version` would be 1.19.15-gke.1300, 1.20.11-gke.1300, and 1.21.5-gke.1300 respectively.
    /// A `machine_type` that has more than 16 GiB of memory is also recommended.
    /// GCFS must be enabled in order to use [image streaming](https://cloud.google.com/kubernetes-engine/docs/how-to/image-streaming).
    /// Structure is documented below.
    /// 
    #[builder(into, default)]
    #[serde(rename = "gcfsConfig")]
    pub r#gcfs_config: Box<Option<super::super::types::container::ClusterNodeConfigGcfsConfig>>,
    /// List of the type and count of accelerator cards attached to the instance.
    /// Structure documented below.
    #[builder(into, default)]
    #[serde(rename = "guestAccelerators")]
    pub r#guest_accelerators: Box<Option<Vec<super::super::types::container::ClusterNodeConfigGuestAccelerator>>>,
    /// Google Virtual NIC (gVNIC) is a virtual network interface.
    /// Installing the gVNIC driver allows for more efficient traffic transmission across the Google network infrastructure.
    /// gVNIC is an alternative to the virtIO-based ethernet driver. GKE nodes must use a Container-Optimized OS node image.
    /// GKE node version 1.15.11-gke.15 or later
    /// Structure is documented below.
    /// 
    /// 
    #[builder(into, default)]
    #[serde(rename = "gvnic")]
    pub r#gvnic: Box<Option<super::super::types::container::ClusterNodeConfigGvnic>>,
    /// The maintenance policy for the hosts on which the GKE VMs run on.
    #[builder(into, default)]
    #[serde(rename = "hostMaintenancePolicy")]
    pub r#host_maintenance_policy: Box<Option<super::super::types::container::ClusterNodeConfigHostMaintenancePolicy>>,
    /// The image type to use for this node. Note that changing the image type
    /// will delete and recreate all nodes in the node pool.
    #[builder(into, default)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<Option<String>>,
    /// Kubelet configuration, currently supported attributes can be found [here](https://cloud.google.com/sdk/gcloud/reference/beta/container/node-pools/create#--system-config-from-file).
    /// Structure is documented below.
    /// 
    /// ```sh
    /// kubelet_config {
    /// cpu_manager_policy   = "static"
    /// cpu_cfs_quota        = true
    /// cpu_cfs_quota_period = "100us"
    /// pod_pids_limit       = 1024
    /// }
    /// ```
    #[builder(into, default)]
    #[serde(rename = "kubeletConfig")]
    pub r#kubelet_config: Box<Option<super::super::types::container::ClusterNodeConfigKubeletConfig>>,
    /// The Kubernetes labels (key/value pairs) to be applied to each node. The kubernetes.io/ and k8s.io/ prefixes are
    /// reserved by Kubernetes Core components and cannot be specified.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// Parameters that can be configured on Linux nodes. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "linuxNodeConfig")]
    pub r#linux_node_config: Box<Option<super::super::types::container::ClusterNodeConfigLinuxNodeConfig>>,
    /// Parameters for the local NVMe SSDs. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "localNvmeSsdBlockConfig")]
    pub r#local_nvme_ssd_block_config: Box<Option<super::super::types::container::ClusterNodeConfigLocalNvmeSsdBlockConfig>>,
    /// The amount of local SSD disks that will be
    /// attached to each cluster node. Defaults to 0.
    #[builder(into, default)]
    #[serde(rename = "localSsdCount")]
    pub r#local_ssd_count: Box<Option<i32>>,
    /// Possible Local SSD encryption modes:
    /// Accepted values are:
    /// * `STANDARD_ENCRYPTION`: The given node will be encrypted using keys managed by Google infrastructure and the keys wll be deleted when the node is deleted.
    /// * `EPHEMERAL_KEY_ENCRYPTION`: The given node will opt-in for using ephemeral key for encrypting Local SSDs. The Local SSDs will not be able to recover data in case of node crash.
    #[builder(into, default)]
    #[serde(rename = "localSsdEncryptionMode")]
    pub r#local_ssd_encryption_mode: Box<Option<String>>,
    /// Parameter for specifying the type of logging agent used in a node pool. This will override any cluster-wide default value. Valid values include DEFAULT and MAX_THROUGHPUT. See [Increasing logging agent throughput](https://cloud.google.com/stackdriver/docs/solutions/gke/managing-logs#throughput) for more information.
    #[builder(into, default)]
    #[serde(rename = "loggingVariant")]
    pub r#logging_variant: Box<Option<String>>,
    /// The name of a Google Compute Engine machine type.
    /// Defaults to `e2-medium`. To create a custom machine type, value should be set as specified
    /// [here](https://cloud.google.com/compute/docs/reference/latest/instances#machineType).
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// The metadata key/value pairs assigned to instances in
    /// the cluster. From GKE `1.12` onwards, `disable-legacy-endpoints` is set to
    /// `true` by the API; if `metadata` is set but that default value is not
    /// included, the provider will attempt to unset the value. To avoid this, set the
    /// value in your config.
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<std::collections::HashMap<String, String>>>,
    /// Minimum CPU platform to be used by this instance.
    /// The instance may be scheduled on the specified or newer CPU platform. Applicable
    /// values are the friendly names of CPU platforms, such as `Intel Haswell`. See the
    /// [official documentation](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
    /// for more information.
    #[builder(into, default)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<Option<String>>,
    /// Setting this field will assign instances of this pool to run on the specified node group. This is useful for running workloads on [sole tenant nodes](https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes).
    #[builder(into, default)]
    #[serde(rename = "nodeGroup")]
    pub r#node_group: Box<Option<String>>,
    /// The set of Google API scopes to be made available
    /// on all of the node VMs under the "default" service account.
    /// Use the "https://www.googleapis.com/auth/cloud-platform" scope to grant access to all APIs. It is recommended that you set `service_account` to a non-default service account and grant IAM roles to that service account for only the resources that it needs.
    /// 
    /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/how-to/access-scopes) for information on migrating off of legacy access scopes.
    #[builder(into, default)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Option<Vec<String>>>,
    /// A boolean that represents whether or not the underlying node VMs
    /// are preemptible. See the [official documentation](https://cloud.google.com/container-engine/docs/preemptible-vm)
    /// for more information. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "preemptible")]
    pub r#preemptible: Box<Option<bool>>,
    /// The configuration of the desired reservation which instances could take capacity from. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "reservationAffinity")]
    pub r#reservation_affinity: Box<Option<super::super::types::container::ClusterNodeConfigReservationAffinity>>,
    /// The GCP labels (key/value pairs) to be applied to each node. Refer [here](https://cloud.google.com/kubernetes-engine/docs/how-to/creating-managing-labels)
    /// for how these labels are applied to clusters, node pools and nodes.
    #[builder(into, default)]
    #[serde(rename = "resourceLabels")]
    pub r#resource_labels: Box<Option<std::collections::HashMap<String, String>>>,
    /// A map of resource manager tag keys and values to be attached to the nodes for managing Compute Engine firewalls using Network Firewall Policies. Tags must be according to specifications found [here](https://cloud.google.com/vpc/docs/tags-firewalls-overview#specifications). A maximum of 5 tag key-value pairs can be specified. Existing tags will be replaced with new values. Tags must be in one of the following formats ([KEY]=[VALUE]) 1. `tagKeys/{tag_key_id}=tagValues/{tag_value_id}` 2. `{org_id}/{tag_key_name}={tag_value_name}` 3. `{project_id}/{tag_key_name}={tag_value_name}`.
    #[builder(into, default)]
    #[serde(rename = "resourceManagerTags")]
    pub r#resource_manager_tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Sandbox configuration for this node.
    #[builder(into, default)]
    #[serde(rename = "sandboxConfig")]
    pub r#sandbox_config: Box<Option<super::super::types::container::ClusterNodeConfigSandboxConfig>>,
    /// Parameters for secondary boot disks to preload container images and data on new nodes. Structure is documented below. `gcfs_config` must be `enabled=true` for this feature to work. `min_master_version` must also be set to use GKE 1.28.3-gke.106700 or later versions.
    #[builder(into, default)]
    #[serde(rename = "secondaryBootDisks")]
    pub r#secondary_boot_disks: Box<Option<Vec<super::super::types::container::ClusterNodeConfigSecondaryBootDisk>>>,
    /// The service account to be used by the Node VMs.
    /// If not specified, the "default" service account is used.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// Shielded Instance options. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Box<Option<super::super::types::container::ClusterNodeConfigShieldedInstanceConfig>>,
    /// Allows specifying multiple [node affinities](https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes#node_affinity_and_anti-affinity) useful for running workloads on [sole tenant nodes](https://cloud.google.com/kubernetes-engine/docs/how-to/sole-tenancy). `node_affinity` structure is documented below.
    /// 
    #[builder(into, default)]
    #[serde(rename = "soleTenantConfig")]
    pub r#sole_tenant_config: Box<Option<super::super::types::container::ClusterNodeConfigSoleTenantConfig>>,
    /// A boolean that represents whether the underlying node VMs are spot.
    /// See the [official documentation](https://cloud.google.com/kubernetes-engine/docs/concepts/spot-vms)
    /// for more information. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "spot")]
    pub r#spot: Box<Option<bool>>,
    /// The list of Storage Pools where boot disks are provisioned.
    #[builder(into, default)]
    #[serde(rename = "storagePools")]
    pub r#storage_pools: Box<Option<Vec<String>>>,
    /// The list of instance tags applied to all nodes. Tags are used to identify
    /// valid sources or targets for network firewalls.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// A list of [Kubernetes taints](https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/)
    /// to apply to nodes. GKE's API can only set this field on cluster creation.
    /// However, GKE will add taints to your nodes if you enable certain features such
    /// as GPUs. If this field is set, any diffs on this field will cause the provider to
    /// recreate the underlying resource. Taint values can be updated safely in
    /// Kubernetes (eg. through `kubectl`), and it's recommended that you do not use
    /// this field to manage taints. If you do, `lifecycle.ignore_changes` is
    /// recommended. Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "taints")]
    pub r#taints: Box<Option<Vec<super::super::types::container::ClusterNodeConfigTaint>>>,
    /// Metadata configuration to expose to workloads on the node pool.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "workloadMetadataConfig")]
    pub r#workload_metadata_config: Box<Option<super::super::types::container::ClusterNodeConfigWorkloadMetadataConfig>>,
}
