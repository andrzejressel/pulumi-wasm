#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NetworkSimStaticIpConfiguration {
    /// The ID of attached data network on which the static IP address will be used. The combination of attached data network and slice defines the network scope of the IP address.
    #[builder(into)]
    #[serde(rename = "attachedDataNetworkId")]
    pub r#attached_data_network_id: Box<String>,
    /// The ID of network slice on which the static IP address will be used. The combination of attached data network and slice defines the network scope of the IP address.
    #[builder(into)]
    #[serde(rename = "sliceId")]
    pub r#slice_id: Box<String>,
    /// The IPv4 address assigned to the SIM at this network scope. This address must be in the userEquipmentStaticAddressPoolPrefix defined in the attached data network.
    #[builder(into, default)]
    #[serde(rename = "staticIpv4Address")]
    pub r#static_ipv_4_address: Box<Option<String>>,
}
