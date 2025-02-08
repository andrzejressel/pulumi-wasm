#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct LoadBalancerLocationStrategy {
    /// Determines the authoritative location when ECS is not preferred, does not exist in the request, or its GeoIP lookup is unsuccessful. Value `pop` will use the Cloudflare PoP location. Value `resolver_ip` will use the DNS resolver GeoIP location. If the GeoIP lookup is unsuccessful, it will use the Cloudflare PoP location. Available values: `pop`, `resolver_ip`. Defaults to `pop`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Whether the EDNS Client Subnet (ECS) GeoIP should be preferred as the authoritative location. Value `always` will always prefer ECS, `never` will never prefer ECS, `proximity` will prefer ECS only when `steering_policy="proximity"`, and `geo` will prefer ECS only when `steering_policy="geo"`. Available values: `always`, `never`, `proximity`, `geo`. Defaults to `proximity`.
    #[builder(into, default)]
    #[serde(rename = "preferEcs")]
    pub r#prefer_ecs: Box<Option<String>>,
}
