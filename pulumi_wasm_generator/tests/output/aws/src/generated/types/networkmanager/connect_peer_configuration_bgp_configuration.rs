#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectPeerConfigurationBgpConfiguration {
    /// A Connect peer core network address.
    #[builder(into, default)]
    #[serde(rename = "coreNetworkAddress")]
    pub r#core_network_address: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "coreNetworkAsn")]
    pub r#core_network_asn: Box<Option<i32>>,
    /// The Connect peer address.
    /// 
    /// The following arguments are optional:
    #[builder(into, default)]
    #[serde(rename = "peerAddress")]
    pub r#peer_address: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "peerAsn")]
    pub r#peer_asn: Box<Option<i32>>,
}
