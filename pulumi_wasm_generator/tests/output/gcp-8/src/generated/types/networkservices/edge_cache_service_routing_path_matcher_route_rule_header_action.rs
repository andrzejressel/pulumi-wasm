#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderAction {
    /// Describes a header to add.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestHeaderToAdds")]
    pub r#request_header_to_adds: Box<Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionRequestHeaderToAdd>>>,
    /// A list of header names for headers that need to be removed from the request prior to forwarding the request to the origin.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "requestHeaderToRemoves")]
    pub r#request_header_to_removes: Box<Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionRequestHeaderToRemove>>>,
    /// Headers to add to the response prior to sending it back to the client.
    /// Response headers are only sent to the client, and do not have an effect on the cache serving the response.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "responseHeaderToAdds")]
    pub r#response_header_to_adds: Box<Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionResponseHeaderToAdd>>>,
    /// A list of header names for headers that need to be removed from the request prior to forwarding the request to the origin.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "responseHeaderToRemoves")]
    pub r#response_header_to_removes: Box<Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleHeaderActionResponseHeaderToRemove>>>,
}
