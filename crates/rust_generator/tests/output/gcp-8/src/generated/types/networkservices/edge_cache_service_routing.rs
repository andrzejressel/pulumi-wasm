#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EdgeCacheServiceRouting {
    /// The list of hostRules to match against. These rules define which hostnames the EdgeCacheService will match against, and which route configurations apply.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "hostRules")]
    pub r#host_rules: Box<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingHostRule>>,
    /// The list of pathMatchers referenced via name by hostRules. PathMatcher is used to match the path portion of the URL when a HostRule matches the URL's host portion.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "pathMatchers")]
    pub r#path_matchers: Box<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcher>>,
}
