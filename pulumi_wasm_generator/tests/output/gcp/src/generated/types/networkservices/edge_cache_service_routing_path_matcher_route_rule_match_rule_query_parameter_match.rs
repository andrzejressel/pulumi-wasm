#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleQueryParameterMatch {
    /// The queryParameterMatch matches if the value of the parameter exactly matches the contents of exactMatch.
    #[builder(into, default)]
    #[serde(rename = "exactMatch")]
    pub r#exact_match: Box<Option<String>>,
    /// The name of the query parameter to match. The query parameter must exist in the request, in the absence of which the request match fails.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies that the queryParameterMatch matches if the request contains the query parameter, irrespective of whether the parameter has a value or not.
    #[builder(into, default)]
    #[serde(rename = "presentMatch")]
    pub r#present_match: Box<Option<bool>>,
}
