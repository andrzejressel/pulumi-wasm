#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UrlMapPathMatcherDefaultRouteActionWeightedBackendService {
    /// The full or partial URL to the default BackendService resource. Before forwarding the
    /// request to backendService, the loadbalancer applies any relevant headerActions
    /// specified as part of this backendServiceWeight.
    #[builder(into, default)]
    #[serde(rename = "backendService")]
    pub r#backend_service: Box<Option<String>>,
    /// Specifies changes to request and response headers that need to take effect for
    /// the selected backendService.
    /// headerAction specified here take effect before headerAction in the enclosing
    /// HttpRouteRule, PathMatcher and UrlMap.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerAction")]
    pub r#header_action: Box<Option<super::super::types::compute::UrlMapPathMatcherDefaultRouteActionWeightedBackendServiceHeaderAction>>,
    /// Specifies the fraction of traffic sent to backendService, computed as
    /// weight / (sum of all weightedBackendService weights in routeAction) .
    /// The selection of a backend service is determined only for new traffic. Once a user's request
    /// has been directed to a backendService, subsequent requests will be sent to the same backendService
    /// as determined by the BackendService's session affinity policy.
    /// The value must be between 0 and 1000
    #[builder(into, default)]
    #[serde(rename = "weight")]
    pub r#weight: Box<Option<i32>>,
}
