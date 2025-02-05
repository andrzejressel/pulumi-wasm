#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkstationConfigHostGceInstance {
    /// An accelerator card attached to the instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "accelerators")]
    pub r#accelerators: Box<Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceAccelerator>>>,
    /// A list of the boost configurations that workstations created using this workstation configuration are allowed to use.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "boostConfigs")]
    pub r#boost_configs: Box<Option<Vec<super::super::types::workstations::WorkstationConfigHostGceInstanceBoostConfig>>>,
    /// Size of the boot disk in GB.
    #[builder(into, default)]
    #[serde(rename = "bootDiskSizeGb")]
    pub r#boot_disk_size_gb: Box<Option<i32>>,
    /// A set of Compute Engine Confidential VM instance options.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "confidentialInstanceConfig")]
    pub r#confidential_instance_config: Box<Option<super::super::types::workstations::WorkstationConfigHostGceInstanceConfidentialInstanceConfig>>,
    /// Whether instances have no public IP address.
    #[builder(into, default)]
    #[serde(rename = "disablePublicIpAddresses")]
    pub r#disable_public_ip_addresses: Box<Option<bool>>,
    /// Whether to disable SSH access to the VM.
    #[builder(into, default)]
    #[serde(rename = "disableSsh")]
    pub r#disable_ssh: Box<Option<bool>>,
    /// Whether to enable nested virtualization on the Compute Engine VMs backing the Workstations.
    /// See https://cloud.google.com/workstations/docs/reference/rest/v1beta/projects.locations.workstationClusters.workstationConfigs#GceInstance.FIELDS.enable_nested_virtualization
    #[builder(into, default)]
    #[serde(rename = "enableNestedVirtualization")]
    pub r#enable_nested_virtualization: Box<Option<bool>>,
    /// The name of a Compute Engine machine type.
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// Number of instances to pool for faster workstation startup.
    #[builder(into, default)]
    #[serde(rename = "poolSize")]
    pub r#pool_size: Box<Option<i32>>,
    /// Email address of the service account that will be used on VM instances used to support this config. This service account must have permission to pull the specified container image. If not set, VMs will run without a service account, in which case the image must be publicly accessible.
    #[builder(into, default)]
    #[serde(rename = "serviceAccount")]
    pub r#service_account: Box<Option<String>>,
    /// Scopes to grant to the service_account. Various scopes are automatically added based on feature usage. When specified, users of workstations under this configuration must have `iam.serviceAccounts.actAs` on the service account.
    #[builder(into, default)]
    #[serde(rename = "serviceAccountScopes")]
    pub r#service_account_scopes: Box<Option<Vec<String>>>,
    /// A set of Compute Engine Shielded instance options.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Box<Option<super::super::types::workstations::WorkstationConfigHostGceInstanceShieldedInstanceConfig>>,
    /// Network tags to add to the Compute Engine machines backing the Workstations.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// Resource manager tags to be bound to the VM instances backing the Workstations.
    /// Tag keys and values have the same definition as
    /// https://cloud.google.com/resource-manager/docs/tags/tags-overview
    /// Keys must be in the format `tagKeys/{tag_key_id}`, and
    /// values are in the format `tagValues/456`.
    #[builder(into, default)]
    #[serde(rename = "vmTags")]
    pub r#vm_tags: Box<Option<std::collections::HashMap<String, String>>>,
}
