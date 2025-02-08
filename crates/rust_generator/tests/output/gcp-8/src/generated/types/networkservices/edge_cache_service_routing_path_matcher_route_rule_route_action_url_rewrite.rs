#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionUrlRewrite {
    /// Prior to forwarding the request to the selected origin, the request's host header is replaced with contents of hostRewrite.
    #[builder(into, default)]
    #[serde(rename = "hostRewrite")]
    pub r#host_rewrite: Box<Option<String>>,
    /// Prior to forwarding the request to the selected origin, the matching portion of the request's path is replaced by pathPrefixRewrite.
    #[builder(into, default)]
    #[serde(rename = "pathPrefixRewrite")]
    pub r#path_prefix_rewrite: Box<Option<String>>,
    /// Prior to forwarding the request to the selected origin, if the
    /// request matched a pathTemplateMatch, the matching portion of the
    /// request's path is replaced re-written using the pattern specified
    /// by pathTemplateRewrite.
    /// pathTemplateRewrite must be between 1 and 255 characters
    /// (inclusive), must start with a '/', and must only use variables
    /// captured by the route's pathTemplate matchers.
    /// pathTemplateRewrite may only be used when all of a route's
    /// MatchRules specify pathTemplate.
    /// Only one of pathPrefixRewrite and pathTemplateRewrite may be
    /// specified.
    #[builder(into, default)]
    #[serde(rename = "pathTemplateRewrite")]
    pub r#path_template_rewrite: Box<Option<String>>,
}
