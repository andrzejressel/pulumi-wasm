#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkGatewayCustomRoute {
    /// A list of address blocks reserved for this virtual network in CIDR notation.
    #[builder(into, default)]
    #[serde(rename = "addressPrefixes")]
    pub r#address_prefixes: Box<Option<Vec<String>>>,
}