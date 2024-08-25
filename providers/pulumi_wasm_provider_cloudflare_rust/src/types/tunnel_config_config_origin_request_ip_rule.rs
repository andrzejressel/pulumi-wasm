#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TunnelConfigConfigOriginRequestIpRule {
    /// Whether to allow the IP prefix.
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    /// Ports to use within the IP rule.
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    /// IP rule prefix.
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
