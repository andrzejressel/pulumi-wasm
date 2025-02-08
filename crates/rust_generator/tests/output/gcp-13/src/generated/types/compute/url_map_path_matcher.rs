#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UrlMapPathMatcher {
    /// defaultCustomErrorResponsePolicy specifies how the Load Balancer returns error responses when BackendService or BackendBucket responds with an error.
    /// This policy takes effect at the PathMatcher level and applies only when no policy has been defined for the error code at lower levels like RouteRule and PathRule within this PathMatcher. If an error code does not have a policy defined in defaultCustomErrorResponsePolicy, then a policy defined for the error code in UrlMap.defaultCustomErrorResponsePolicy takes effect.
    /// For example, consider a UrlMap with the following configuration:
    /// UrlMap.defaultCustomErrorResponsePolicy is configured with policies for 5xx and 4xx errors
    /// A RouteRule for /coming_soon/ is configured for the error code 404.
    /// If the request is for www.myotherdomain.com and a 404 is encountered, the policy under UrlMap.defaultCustomErrorResponsePolicy takes effect. If a 404 response is encountered for the request www.example.com/current_events/, the pathMatcher's policy takes effect. If however, the request for www.example.com/coming_soon/ encounters a 404, the policy in RouteRule.customErrorResponsePolicy takes effect. If any of the requests in this example encounter a 500 error code, the policy at UrlMap.defaultCustomErrorResponsePolicy takes effect.
    /// When used in conjunction with pathMatcher.defaultRouteAction.retryPolicy, retries take precedence. Only once all retries are exhausted, the defaultCustomErrorResponsePolicy is applied. While attempting a retry, if load balancer is successful in reaching the service, the defaultCustomErrorResponsePolicy is ignored and the response from the service is returned to the client.
    /// defaultCustomErrorResponsePolicy is supported only for global external Application Load Balancers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "defaultCustomErrorResponsePolicy")]
    pub r#default_custom_error_response_policy: Box<Option<super::super::types::compute::UrlMapPathMatcherDefaultCustomErrorResponsePolicy>>,
    /// defaultRouteAction takes effect when none of the pathRules or routeRules match. The load balancer performs
    /// advanced routing actions like URL rewrites, header transformations, etc. prior to forwarding the request
    /// to the selected backend. If defaultRouteAction specifies any weightedBackendServices, defaultService must not be set.
    /// Conversely if defaultService is set, defaultRouteAction cannot contain any weightedBackendServices.
    /// Only one of defaultRouteAction or defaultUrlRedirect must be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "defaultRouteAction")]
    pub r#default_route_action: Box<Option<super::super::types::compute::UrlMapPathMatcherDefaultRouteAction>>,
    /// The backend service or backend bucket to use when none of the given paths match.
    #[builder(into, default)]
    #[serde(rename = "defaultService")]
    pub r#default_service: Box<Option<String>>,
    /// When none of the specified hostRules match, the request is redirected to a URL specified
    /// by defaultUrlRedirect. If defaultUrlRedirect is specified, defaultService or
    /// defaultRouteAction must not be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "defaultUrlRedirect")]
    pub r#default_url_redirect: Box<Option<super::super::types::compute::UrlMapPathMatcherDefaultUrlRedirect>>,
    /// An optional description of this resource. Provide this property when you create
    /// the resource.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Specifies changes to request and response headers that need to take effect for
    /// the selected backendService. HeaderAction specified here are applied after the
    /// matching HttpRouteRule HeaderAction and before the HeaderAction in the UrlMap
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Box<Option<super::super::types::compute::UrlMapPathMatcherHeaderAction>>,
    /// The name to which this PathMatcher is referred by the HostRule.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The list of path rules. Use this list instead of routeRules when routing based
    /// on simple path matching is all that's required. The order by which path rules
    /// are specified does not matter. Matches are always done on the longest-path-first
    /// basis. For example: a pathRule with a path /a/b/c/* will match before /a/b/*
    /// irrespective of the order in which those paths appear in this list. Within a
    /// given pathMatcher, only one of pathRules or routeRules must be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "pathRules")]
    pub r#path_rules: Box<Option<Vec<super::super::types::compute::UrlMapPathMatcherPathRule>>>,
    /// The list of ordered HTTP route rules. Use this list instead of pathRules when
    /// advanced route matching and routing actions are desired. The order of specifying
    /// routeRules matters: the first rule that matches will cause its specified routing
    /// action to take effect. Within a given pathMatcher, only one of pathRules or
    /// routeRules must be set. routeRules are not supported in UrlMaps intended for
    /// External load balancers.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "routeRules")]
    pub r#route_rules: Box<Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRule>>>,
}
