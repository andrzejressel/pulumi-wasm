#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UrlMapPathMatcherPathRule {
    /// customErrorResponsePolicy specifies how the Load Balancer returns error responses when BackendService or BackendBucket responds with an error.
    /// If a policy for an error code is not configured for the PathRule, a policy for the error code configured in pathMatcher.defaultCustomErrorResponsePolicy is applied. If one is not specified in pathMatcher.defaultCustomErrorResponsePolicy, the policy configured in UrlMap.defaultCustomErrorResponsePolicy takes effect.
    /// For example, consider a UrlMap with the following configuration:
    /// UrlMap.defaultCustomErrorResponsePolicy are configured with policies for 5xx and 4xx errors
    /// A PathRule for /coming_soon/ is configured for the error code 404.
    /// If the request is for www.myotherdomain.com and a 404 is encountered, the policy under UrlMap.defaultCustomErrorResponsePolicy takes effect. If a 404 response is encountered for the request www.example.com/current_events/, the pathMatcher's policy takes effect. If however, the request for www.example.com/coming_soon/ encounters a 404, the policy in PathRule.customErrorResponsePolicy takes effect. If any of the requests in this example encounter a 500 error code, the policy at UrlMap.defaultCustomErrorResponsePolicy takes effect.
    /// customErrorResponsePolicy is supported only for global external Application Load Balancers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "customErrorResponsePolicy")]
    pub r#custom_error_response_policy: Box<Option<super::super::types::compute::UrlMapPathMatcherPathRuleCustomErrorResponsePolicy>>,
    /// The list of path patterns to match. Each must start with / and the only place a
    /// \* is allowed is at the end following a /. The string fed to the path matcher
    /// does not include any text after the first ? or #, and those chars are not
    /// allowed here.
    #[builder(into)]
    #[serde(rename = "paths")]
    pub r#paths: Box<Vec<String>>,
    /// In response to a matching path, the load balancer performs advanced routing
    /// actions like URL rewrites, header transformations, etc. prior to forwarding the
    /// request to the selected backend. If routeAction specifies any
    /// weightedBackendServices, service must not be set. Conversely if service is set,
    /// routeAction cannot contain any  weightedBackendServices. Only one of routeAction
    /// or urlRedirect must be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "routeAction")]
    pub r#route_action: Box<Option<super::super::types::compute::UrlMapPathMatcherPathRuleRouteAction>>,
    /// The backend service or backend bucket to use if any of the given paths match.
    #[builder(into, default)]
    #[serde(rename = "service")]
    pub r#service: Box<Option<String>>,
    /// When a path pattern is matched, the request is redirected to a URL specified
    /// by urlRedirect. If urlRedirect is specified, service or routeAction must not
    /// be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "urlRedirect")]
    pub r#url_redirect: Box<Option<super::super::types::compute::UrlMapPathMatcherPathRuleUrlRedirect>>,
}
