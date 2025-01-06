#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPublicIpv4PoolPoolAddressRange {
    /// Number of addresses in the range.
    #[builder(into)]
    #[serde(rename = "addressCount")]
    pub r#address_count: Box<i32>,
    /// Number of available addresses in the range.
    #[builder(into)]
    #[serde(rename = "availableAddressCount")]
    pub r#available_address_count: Box<i32>,
    /// First address in the range.
    #[builder(into)]
    #[serde(rename = "firstAddress")]
    pub r#first_address: Box<String>,
    /// Last address in the range.
    #[builder(into)]
    #[serde(rename = "lastAddress")]
    pub r#last_address: Box<String>,
}
