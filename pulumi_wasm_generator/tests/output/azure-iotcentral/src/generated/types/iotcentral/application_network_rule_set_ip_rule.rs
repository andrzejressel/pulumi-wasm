#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationNetworkRuleSetIpRule {
    /// The IP address range in CIDR notation for the IP Rule.
    #[builder(into)]
    #[serde(rename = "ipMask")]
    pub r#ip_mask: Box<String>,
    /// The name of the IP Rule
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
