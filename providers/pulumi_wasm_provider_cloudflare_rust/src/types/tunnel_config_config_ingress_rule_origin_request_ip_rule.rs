#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRuleOriginRequestIpRule {
    #[serde(rename = "allow")]
    pub r#allow: Box<Option<bool>>,
    #[serde(rename = "ports")]
    pub r#ports: Box<Option<Vec<i32>>>,
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
}
