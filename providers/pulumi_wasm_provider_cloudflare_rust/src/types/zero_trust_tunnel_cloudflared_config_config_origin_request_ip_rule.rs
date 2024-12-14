#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ZeroTrustTunnelCloudflaredConfigConfigOriginRequestIpRule {
    /// Whether to allow the IP prefix.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    /// Ports to use within the IP rule.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    /// IP rule prefix.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
