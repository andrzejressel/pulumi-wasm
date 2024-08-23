#[derive(serde::Serialize)]
pub struct TunnelConfigConfigIngressRule {
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequest>>,
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
