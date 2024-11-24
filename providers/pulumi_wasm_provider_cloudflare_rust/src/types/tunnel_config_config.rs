#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct TunnelConfigConfig {
    /// Each incoming request received by cloudflared causes cloudflared to send a request to a local service. This section configures the rules that determine which requests are sent to which local services. Last rule must match all requests, e.g `service = "http_status:503"`. [Read more](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/tunnel-guide/local/local-management/ingress/).
    #[builder(into)]
    #[serde(rename = "ingressRules")]
    pub r#ingress_rules: Box<Vec<crate::types::TunnelConfigConfigIngressRule>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "originRequest")]
    pub r#origin_request: Box<Option<crate::types::TunnelConfigConfigOriginRequest>>,
    /// If you're exposing a [private network](https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/private-net/), you need to add the `warp-routing` key and set it to `true`.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "warpRouting")]
    pub r#warp_routing: Box<Option<crate::types::TunnelConfigConfigWarpRouting>>,
}
