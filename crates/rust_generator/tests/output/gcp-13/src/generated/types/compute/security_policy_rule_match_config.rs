#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecurityPolicyRuleMatchConfig {
    /// CIDR IP address range. Maximum number of srcIpRanges allowed is 10.
    #[builder(into, default)]
    #[serde(rename = "srcIpRanges")]
    pub r#src_ip_ranges: Box<Option<Vec<String>>>,
}
