#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct TunnelConfigConfigIngressRule {
    /// Hostname to match the incoming request with. If the hostname matches, the request will be sent to the service.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigIngressRuleOriginRequest>>,
    /// Path of the incoming request. If the path matches, the request will be sent to the local service.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// Name of the service to which the request will be sent.
    #[builder(into)]
    #[serde(rename = "service")]
    pub r#service: Box<String>,
}
