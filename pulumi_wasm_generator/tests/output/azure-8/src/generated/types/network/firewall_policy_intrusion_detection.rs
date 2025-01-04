#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirewallPolicyIntrusionDetection {
    /// In which mode you want to run intrusion detection: `Off`, `Alert` or `Deny`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// A list of Private IP address ranges to identify traffic direction. By default, only ranges defined by IANA RFC 1918 are considered private IP addresses.
    #[builder(into, default)]
    #[serde(rename = "privateRanges")]
    pub r#private_ranges: Box<Option<Vec<String>>>,
    /// One or more `signature_overrides` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "signatureOverrides")]
    pub r#signature_overrides: Box<Option<Vec<super::super::types::network::FirewallPolicyIntrusionDetectionSignatureOverride>>>,
    /// One or more `traffic_bypass` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "trafficBypasses")]
    pub r#traffic_bypasses: Box<Option<Vec<super::super::types::network::FirewallPolicyIntrusionDetectionTrafficBypass>>>,
}
