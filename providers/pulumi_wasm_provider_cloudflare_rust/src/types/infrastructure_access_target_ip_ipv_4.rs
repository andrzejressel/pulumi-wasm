#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct InfrastructureAccessTargetIpIpv4 {
    /// The IP address of the target.
    #[builder(into)]
    #[serde(rename = "ipAddr")]
    pub r#ip_addr: Box<String>,
    /// The private virtual network identifier for the target.
    #[builder(into)]
    #[serde(rename = "virtualNetworkId")]
    pub r#virtual_network_id: Box<String>,
}
