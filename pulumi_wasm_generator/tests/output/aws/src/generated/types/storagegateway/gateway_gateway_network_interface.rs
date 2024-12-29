#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GatewayGatewayNetworkInterface {
    /// The Internet Protocol version 4 (IPv4) address of the interface.
    #[builder(into, default)]
    #[serde(rename = "ipv4Address")]
    pub r#ipv_4_address: Box<Option<String>>,
}
