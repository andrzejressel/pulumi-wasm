#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct UrlMapPathMatcherRouteRuleHeaderAction {
    /// Headers to add to a matching request prior to forwarding the request to the backendService.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestHeadersToAdds")]
    pub r#request_headers_to_adds: Box<Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleHeaderActionRequestHeadersToAdd>>>,
    /// A list of header names for headers that need to be removed from the request prior to
    /// forwarding the request to the backendService.
    #[builder(into, default)]
    #[serde(rename = "requestHeadersToRemoves")]
    pub r#request_headers_to_removes: Box<Option<Vec<String>>>,
    /// Headers to add the response prior to sending the response back to the client.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "responseHeadersToAdds")]
    pub r#response_headers_to_adds: Box<Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleHeaderActionResponseHeadersToAdd>>>,
    /// A list of header names for headers that need to be removed from the response prior to sending the
    /// response back to the client.
    #[builder(into, default)]
    #[serde(rename = "responseHeadersToRemoves")]
    pub r#response_headers_to_removes: Box<Option<Vec<String>>>,
}
