#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfigurationIcmpTypeCode {
    /// ICMP code. A value of `-1` means all codes for the specified ICMP type.
    #[builder(into)]
    #[serde(rename = "code")]
    pub r#code: Box<i32>,
    /// ICMP type. A value of `-1` means all types.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<i32>,
}
