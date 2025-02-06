#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ModuleNetworkProfile {
    /// The private IPv4 address of the network interface. Changing this forces a new Dedicated Hardware Security Module to be created.
    #[builder(into)]
    #[serde(rename = "networkInterfacePrivateIpAddresses")]
    pub r#network_interface_private_ip_addresses: Box<Vec<String>>,
    /// The ID of the subnet. Changing this forces a new Dedicated Hardware Security Module to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
