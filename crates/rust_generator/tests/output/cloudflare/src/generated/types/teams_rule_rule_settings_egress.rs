#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TeamsRuleRuleSettingsEgress {
    /// The IPv4 address to be used for egress.
    #[builder(into)]
    #[serde(rename = "ipv4")]
    pub r#ipv_4: Box<String>,
    /// The IPv4 address to be used for egress in the event of an error egressing with the primary IPv4. Can be '0.0.0.0' to indicate local egreass via Warp IPs.
    #[builder(into, default)]
    #[serde(rename = "ipv4Fallback")]
    pub r#ipv_4_fallback: Box<Option<String>>,
    /// The IPv6 range to be used for egress.
    #[builder(into)]
    #[serde(rename = "ipv6")]
    pub r#ipv_6: Box<String>,
}
