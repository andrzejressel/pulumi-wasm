#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HciNetworkInterfaceIpConfiguration {
    /// The IPv4 address of the gateway for the Network Interface.
    #[builder(into, default)]
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
    /// The prefix length for the address of the Network Interface.
    #[builder(into, default)]
    #[serde(rename = "prefixLength")]
    pub r#prefix_length: Box<Option<String>>,
    /// The IPv4 address of the IP configuration. Changing this forces a new resource to be created.
    #[builder(into, default)]
    #[serde(rename = "privateIpAddress")]
    pub r#private_ip_address: Box<Option<String>>,
    /// The resource ID of the Stack HCI Logical Network bound to the IP configuration. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
