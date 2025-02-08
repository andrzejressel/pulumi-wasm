#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetVirtualMachineScaleSetInstance {
    /// The Hostname of this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "computerName")]
    pub r#computer_name: Box<String>,
    /// The Instance ID of this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "instanceId")]
    pub r#instance_id: Box<String>,
    /// Whether the latest model has been applied to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "latestModelApplied")]
    pub r#latest_model_applied: Box<bool>,
    /// The name of this Virtual Machine Scale Set.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The power state of the virtual machine.
    #[builder(into)]
    #[serde(rename = "powerState")]
    pub r#power_state: Box<String>,
    /// The Primary Private IP Address assigned to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<String>,
    /// A list of Private IP Addresses assigned to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "privateIpAddresses")]
    pub r#private_ip_addresses: Box<Vec<String>>,
    /// The virtual machines scale set IP Configuration's PublicIPAddress configuration. The `public_ip_address` is documented below.
    #[builder(into)]
    #[serde(rename = "publicIpAddress")]
    pub r#public_ip_address: Box<String>,
    /// A list of the Public IP Addresses assigned to this Virtual Machine.
    #[builder(into)]
    #[serde(rename = "publicIpAddresses")]
    pub r#public_ip_addresses: Box<Vec<String>>,
    /// The unique ID of the virtual machine.
    #[builder(into)]
    #[serde(rename = "virtualMachineId")]
    pub r#virtual_machine_id: Box<String>,
    /// The zones of the virtual machine.
    #[builder(into)]
    #[serde(rename = "zone")]
    pub r#zone: Box<String>,
}
