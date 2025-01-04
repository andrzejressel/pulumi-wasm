#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsVirtualMachineScaleSetNetworkInterface {
    /// A list of IP Addresses of DNS Servers which should be assigned to the Network Interface.
    #[builder(into, default)]
    #[serde(rename = "dnsServers")]
    pub r#dns_servers: Box<Option<Vec<String>>>,
    /// Does this Network Interface support Accelerated Networking? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enableAcceleratedNetworking")]
    pub r#enable_accelerated_networking: Box<Option<bool>>,
    /// Does this Network Interface support IP Forwarding? Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "enableIpForwarding")]
    pub r#enable_ip_forwarding: Box<Option<bool>>,
    /// One or more `ip_configuration` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Box<Vec<super::super::types::compute::WindowsVirtualMachineScaleSetNetworkInterfaceIpConfiguration>>,
    /// The Name which should be used for this Network Interface. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The ID of a Network Security Group which should be assigned to this Network Interface.
    #[builder(into, default)]
    #[serde(rename = "networkSecurityGroupId")]
    pub r#network_security_group_id: Box<Option<String>>,
    /// Is this the Primary IP Configuration?
    /// 
    /// > **Note:** If multiple `network_interface` blocks are specified, one must be set to `primary`.
    #[builder(into, default)]
    #[serde(rename = "primary")]
    pub r#primary: Box<Option<bool>>,
}
