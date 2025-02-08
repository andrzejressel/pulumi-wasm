#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceGceSetup {
    /// The hardware accelerators used on this instance. If you use accelerators, make sure that your configuration has
    /// [enough vCPUs and memory to support the `machine_type` you have selected](https://cloud.google.com/compute/docs/gpus/#gpus-list).
    /// Currently supports only one accelerator configuration.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "acceleratorConfigs")]
    pub r#accelerator_configs: Box<Option<Vec<super::super::types::workbench::InstanceGceSetupAcceleratorConfig>>>,
    /// The definition of a boot disk.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "bootDisk")]
    pub r#boot_disk: Box<Option<super::super::types::workbench::InstanceGceSetupBootDisk>>,
    /// Use a container image to start the workbench instance.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "containerImage")]
    pub r#container_image: Box<Option<super::super::types::workbench::InstanceGceSetupContainerImage>>,
    /// Data disks attached to the VM instance. Currently supports only one data disk.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "dataDisks")]
    pub r#data_disks: Box<Option<super::super::types::workbench::InstanceGceSetupDataDisks>>,
    /// Optional. If true, no external IP will be assigned to this VM instance.
    #[builder(into, default)]
    #[serde(rename = "disablePublicIp")]
    pub r#disable_public_ip: Box<Option<bool>>,
    /// Optional. Flag to enable ip forwarding or not, default false/off.
    /// https://cloud.google.com/vpc/docs/using-routes#canipforward
    #[builder(into, default)]
    #[serde(rename = "enableIpForwarding")]
    pub r#enable_ip_forwarding: Box<Option<bool>>,
    /// Optional. The machine type of the VM instance. https://cloud.google.com/compute/docs/machine-resource
    #[builder(into, default)]
    #[serde(rename = "machineType")]
    pub r#machine_type: Box<Option<String>>,
    /// Optional. Custom metadata to apply to this instance.
    #[builder(into, default)]
    #[serde(rename = "metadata")]
    pub r#metadata: Box<Option<std::collections::HashMap<String, String>>>,
    /// The network interfaces for the VM. Supports only one interface.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Box<Option<Vec<super::super::types::workbench::InstanceGceSetupNetworkInterface>>>,
    /// The service account that serves as an identity for the VM instance. Currently supports only one service account.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "serviceAccounts")]
    pub r#service_accounts: Box<Option<Vec<super::super::types::workbench::InstanceGceSetupServiceAccount>>>,
    /// A set of Shielded Instance options. See [Images using supported Shielded
    /// VM features](https://cloud.google.com/compute/docs/instances/modifying-shielded-vm).
    /// Not all combinations are valid.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "shieldedInstanceConfig")]
    pub r#shielded_instance_config: Box<Option<super::super::types::workbench::InstanceGceSetupShieldedInstanceConfig>>,
    /// Optional. The Compute Engine tags to add to instance (see [Tagging
    /// instances](https://cloud.google.com/compute/docs/label-or-tag-resources#tags)).
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<Vec<String>>>,
    /// Definition of a custom Compute Engine virtual machine image for starting
    /// a workbench instance with the environment installed directly on the VM.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "vmImage")]
    pub r#vm_image: Box<Option<super::super::types::workbench::InstanceGceSetupVmImage>>,
}
