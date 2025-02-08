#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RegionUrlMapPathMatcherRouteRuleRouteActionWeightedBackendService {
    /// The full or partial URL to the default BackendService resource. Before forwarding the request to backendService, the load balancer applies any relevant headerActions specified as part of this backendServiceWeight.
    #[builder(into)]
    #[serde(rename = "backendService")]
    pub r#backend_service: Box<String>,
    /// Specifies changes to request and response headers that need to take effect for the selected backendService.
    /// headerAction specified here take effect before headerAction in the enclosing HttpRouteRule, PathMatcher and UrlMap.
    /// headerAction is not supported for load balancers that have their loadBalancingScheme set to EXTERNAL.
    /// Not supported when the URL map is bound to a target gRPC proxy that has validateForProxyless field set to true.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Box<Option<super::super::types::compute::RegionUrlMapPathMatcherRouteRuleRouteActionWeightedBackendServiceHeaderAction>>,
    /// Specifies the fraction of traffic sent to a backend service, computed as weight / (sum of all weightedBackendService weights in routeAction) .
    /// The selection of a backend service is determined only for new traffic. Once a user's request has been directed to a backend service, subsequent requests are sent to the same backend service as determined by the backend service's session affinity policy.
    /// The value must be from 0 to 1000.
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<i32>,
}
