#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcher {
    /// A human-readable description of the resource.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The name to which this PathMatcher is referred by the HostRule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The routeRules to match against. routeRules support advanced routing behaviour, and can match on paths, headers and query parameters, as well as status codes and HTTP methods.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "routeRules")]
    pub r#route_rules: Box<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRule>>,
}
