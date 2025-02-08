#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetClusterClusterAutoscalingAutoProvisioningDefault {
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached to each node in the node pool.
    #[builder(into)]
    #[serde(rename = "bootDiskKmsKey")]
    pub r#boot_disk_kms_key: Box<String>,
    /// Size of the disk attached to each node, specified in GB. The smallest allowed disk size is 10GB.
    #[builder(into)]
    #[serde(rename = "diskSize")]
    pub r#disk_size: Box<i32>,
    /// Type of the disk attached to each node.
    #[builder(into)]
    #[serde(rename = "diskType")]
    pub r#disk_type: Box<String>,
    /// The default image type used by NAP once a new node pool is being created.
    #[builder(into)]
    #[serde(rename = "imageType")]
    pub r#image_type: Box<String>,
    /// NodeManagement configuration for this NodePool.
    #[builder(into)]
    #[serde(rename = "managements")]
    pub r#managements: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultManagement>>,
    /// Minimum CPU platform to be used by this instance. The instance may be scheduled on the specified or newer CPU platform. Applicable values are the friendly names of CPU platforms, such as Intel Haswell.
    #[builder(into)]
    #[serde(rename = "minCpuPlatform")]
    pub r#min_cpu_platform: Box<String>,
    /// Scopes that are used by NAP when creating node pools.
    #[builder(into)]
    #[serde(rename = "oauthScopes")]
    pub r#oauth_scopes: Box<Vec<String>>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    #[builder(into)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<String>,
    /// Shielded Instance options.
    #[builder(into)]
    #[serde(rename = "shieldedInstanceConfigs")]
    pub r#shielded_instance_configs: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultShieldedInstanceConfig>>,
    /// Specifies the upgrade settings for NAP created node pools
    #[builder(into)]
    #[serde(rename = "upgradeSettings")]
    pub r#upgrade_settings: Box<Vec<super::super::types::container::GetClusterClusterAutoscalingAutoProvisioningDefaultUpgradeSetting>>,
}
