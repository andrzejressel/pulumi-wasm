#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionUrlMapPathMatcherRouteRuleRouteActionWeightedBackendServiceHeaderAction {
    /// Headers to add to a matching request before forwarding the request to the backendService.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestHeadersToAdds")]
    pub r#request_headers_to_adds: Box<Option<Vec<super::super::types::compute::RegionUrlMapPathMatcherRouteRuleRouteActionWeightedBackendServiceHeaderActionRequestHeadersToAdd>>>,
    /// A list of header names for headers that need to be removed from the request before forwarding the request to the backendService.
    #[builder(into, default)]
    #[serde(rename = "requestHeadersToRemoves")]
    pub r#request_headers_to_removes: Box<Option<Vec<String>>>,
    /// Headers to add the response before sending the response back to the client.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "responseHeadersToAdds")]
    pub r#response_headers_to_adds: Box<Option<Vec<super::super::types::compute::RegionUrlMapPathMatcherRouteRuleRouteActionWeightedBackendServiceHeaderActionResponseHeadersToAdd>>>,
    /// A list of header names for headers that need to be removed from the response before sending the response back to the client.
    #[builder(into, default)]
    #[serde(rename = "responseHeadersToRemoves")]
    pub r#response_headers_to_removes: Box<Option<Vec<String>>>,
}
