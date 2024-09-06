#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TunnelConfigConfigIngressRule {
    /// Hostname to match the incoming request with. If the hostname matches, the request will be sent to the service.
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequest>>,
    /// Path of the incoming request. If the path matches, the request will be sent to the local service.
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Name of the service to which the request will be sent.
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
