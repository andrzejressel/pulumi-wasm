#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnConnectionDetailCloudVpn {
    /// (Output)
    /// The created Cloud VPN gateway name.
    #[builder(into, default)]
    #[serde(rename = "gateway")]
    pub r#gateway: Box<Option<String>>,
}
