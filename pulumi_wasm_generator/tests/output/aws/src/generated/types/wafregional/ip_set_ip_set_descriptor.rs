#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IpSetIpSetDescriptor {
    /// The string like IPV4 or IPV6.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The CIDR notation.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
