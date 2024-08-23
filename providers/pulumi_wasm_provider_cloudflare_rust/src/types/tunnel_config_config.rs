#[derive(serde::Serialize)]
pub struct TunnelConfigConfig {
    #[serde(rename = "ingressRules")]
    pub r#ingress_rules: Box<Vec<crate::types::TunnelConfigConfigIngressRule>>,
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigOriginRequest>>,
    #[serde(rename = "warpRouting")]
    pub r#warp_routing: Box<Option<crate::types::TunnelConfigConfigWarpRouting>>,
}
