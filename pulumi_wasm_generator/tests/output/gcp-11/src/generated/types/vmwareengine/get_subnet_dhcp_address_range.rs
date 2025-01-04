#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSubnetDhcpAddressRange {
    /// The first IP address of the range.
    #[builder(into)]
    #[serde(rename = "firstAddress")]
    pub r#first_address: Box<String>,
    /// The last IP address of the range.
    #[builder(into)]
    #[serde(rename = "lastAddress")]
    pub r#last_address: Box<String>,
}
