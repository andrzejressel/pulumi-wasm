#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterMainProfile {
    /// The resource ID of an associated disk encryption set. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "diskEncryptionSetId")]
    pub r#disk_encryption_set_id: Box<Option<String>>,
    /// Whether main virtual machines are encrypted at host. Defaults to `false`. Changing this forces a new resource to be created.
    /// 
    /// > **NOTE:** `encryption_at_host_enabled` is only available for certain VM sizes and the `EncryptionAtHost` feature must be enabled for your subscription. Please see the [Azure documentation](https://learn.microsoft.com/azure/virtual-machines/disks-enable-host-based-encryption-portal?tabs=azure-powershell) for more information.
    #[builder(into, default)]
    #[serde(rename = "encryptionAtHostEnabled")]
    pub r#encryption_at_host_enabled: Box<Option<bool>>,
    /// The ID of the subnet where main nodes will be hosted. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
    /// The size of the Virtual Machines for the main nodes. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "vmSize")]
    pub r#vm_size: Box<String>,
}