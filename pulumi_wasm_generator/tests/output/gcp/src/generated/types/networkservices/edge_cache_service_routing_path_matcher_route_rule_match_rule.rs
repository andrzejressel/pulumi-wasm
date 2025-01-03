#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRule {
    /// For satisfying the matchRule condition, the path of the request must exactly match the value specified in fullPathMatch after removing any query parameters and anchor that may be part of the original URL.
    #[builder(into, default)]
    #[serde(rename = "fullPathMatch")]
    pub r#full_path_match: Box<Option<String>>,
    /// Specifies a list of header match criteria, all of which must match corresponding headers in the request.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerMatches")]
    pub r#header_matches: Box<Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleHeaderMatch>>>,
    /// Specifies that prefixMatch and fullPathMatch matches are case sensitive.
    #[builder(into, default)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Box<Option<bool>>,
    /// For satisfying the matchRule condition, the path of the request
    /// must match the wildcard pattern specified in pathTemplateMatch
    /// after removing any query parameters and anchor that may be part
    /// of the original URL.
    /// pathTemplateMatch must be between 1 and 255 characters
    /// (inclusive).  The pattern specified by pathTemplateMatch may
    /// have at most 5 wildcard operators and at most 5 variable
    /// captures in total.
    #[builder(into, default)]
    #[serde(rename = "pathTemplateMatch")]
    pub r#path_template_match: Box<Option<String>>,
    /// For satisfying the matchRule condition, the request's path must begin with the specified prefixMatch. prefixMatch must begin with a /.
    #[builder(into, default)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Box<Option<String>>,
    /// Specifies a list of query parameter match criteria, all of which must match corresponding query parameters in the request.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "queryParameterMatches")]
    pub r#query_parameter_matches: Box<Option<Vec<super::super::types::networkservices::EdgeCacheServiceRoutingPathMatcherRouteRuleMatchRuleQueryParameterMatch>>>,
}
