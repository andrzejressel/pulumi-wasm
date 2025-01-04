#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionUrlMapPathMatcherPathRuleRouteAction {
    /// The specification for allowing client side cross-origin requests. Please see W3C
    /// Recommendation for Cross Origin Resource Sharing
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "corsPolicy")]
    pub r#cors_policy: Box<Option<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteActionCorsPolicy>>,
    /// The specification for fault injection introduced into traffic to test the
    /// resiliency of clients to backend service failure. As part of fault injection,
    /// when clients send requests to a backend service, delays can be introduced by
    /// Loadbalancer on a percentage of requests before sending those request to the
    /// backend service. Similarly requests from clients can be aborted by the
    /// Loadbalancer for a percentage of requests. timeout and retry_policy will be
    /// ignored by clients that are configured with a fault_injection_policy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "faultInjectionPolicy")]
    pub r#fault_injection_policy: Box<Option<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteActionFaultInjectionPolicy>>,
    /// Specifies the policy on how requests intended for the route's backends are
    /// shadowed to a separate mirrored backend service. Loadbalancer does not wait for
    /// responses from the shadow service. Prior to sending traffic to the shadow
    /// service, the host / authority header is suffixed with -shadow.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestMirrorPolicy")]
    pub r#request_mirror_policy: Box<Option<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteActionRequestMirrorPolicy>>,
    /// Specifies the retry policy associated with this route.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "retryPolicy")]
    pub r#retry_policy: Box<Option<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteActionRetryPolicy>>,
    /// Specifies the timeout for the selected route. Timeout is computed from the time
    /// the request is has been fully processed (i.e. end-of-stream) up until the
    /// response has been completely processed. Timeout includes all retries. If not
    /// specified, the default value is 15 seconds.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "timeout")]
    pub r#timeout: Box<Option<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteActionTimeout>>,
    /// The spec to modify the URL of the request, prior to forwarding the request to
    /// the matched service
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "urlRewrite")]
    pub r#url_rewrite: Box<Option<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteActionUrlRewrite>>,
    /// A list of weighted backend services to send traffic to when a route match
    /// occurs. The weights determine the fraction of traffic that flows to their
    /// corresponding backend service. If all traffic needs to go to a single backend
    /// service, there must be one  weightedBackendService with weight set to a non 0
    /// number. Once a backendService is identified and before forwarding the request to
    /// the backend service, advanced routing actions like Url rewrites and header
    /// transformations are applied depending on additional settings specified in this
    /// HttpRouteAction.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "weightedBackendServices")]
    pub r#weighted_backend_services: Box<Option<Vec<super::super::types::compute::RegionUrlMapPathMatcherPathRuleRouteActionWeightedBackendService>>>,
}
