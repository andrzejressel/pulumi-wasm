#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AddressMapIp {
    /// An IPv4 or IPv6 address.
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
}
