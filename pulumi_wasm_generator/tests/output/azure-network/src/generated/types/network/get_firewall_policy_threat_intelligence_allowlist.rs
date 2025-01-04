#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetFirewallPolicyThreatIntelligenceAllowlist {
    #[builder(into)]
    #[serde(rename = "fqdns")]
    pub r#fqdns: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Vec<String>>,
}
