#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ZeroTrustTunnelCloudflaredConfigConfigIngressRuleOriginRequestIpRule {
    /// Whether to allow the IP prefix.
    #[builder(into, default)]
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    /// Ports to use within the IP rule.
    #[builder(into, default)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    /// IP rule prefix.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
