#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteAction {
    /// The policy to use for defining caching and signed request behaviour for requests that match this route.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cdnPolicy")]
    pub r#cdn_policy: Box<Option<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicy>>,
    /// CORSPolicy defines Cross-Origin-Resource-Sharing configuration, including which CORS response headers will be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "corsPolicy")]
    pub r#cors_policy: Box<Option<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCorsPolicy>>,
    /// The URL rewrite configuration for requests that match this route.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "urlRewrite")]
    pub r#url_rewrite: Box<Option<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionUrlRewrite>>,
}
