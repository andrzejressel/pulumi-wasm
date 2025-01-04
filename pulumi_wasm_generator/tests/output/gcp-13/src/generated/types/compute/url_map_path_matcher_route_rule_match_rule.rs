#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapPathMatcherRouteRuleMatchRule {
    /// For satisfying the matchRule condition, the path of the request must exactly
    /// match the value specified in fullPathMatch after removing any query parameters
    /// and anchor that may be part of the original URL. FullPathMatch must be between 1
    /// and 1024 characters. Only one of prefixMatch, fullPathMatch or regexMatch must
    /// be specified.
    #[builder(into, default)]
    #[serde(rename = "fullPathMatch")]
    pub r#full_path_match: Box<Option<String>>,
    /// Specifies a list of header match criteria, all of which must match corresponding
    /// headers in the request.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headerMatches")]
    pub r#header_matches: Box<Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleMatchRuleHeaderMatch>>>,
    /// Specifies that prefixMatch and fullPathMatch matches are case sensitive.
    /// Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Box<Option<bool>>,
    /// Opaque filter criteria used by Loadbalancer to restrict routing configuration to
    /// a limited set xDS compliant clients. In their xDS requests to Loadbalancer, xDS
    /// clients present node metadata. If a match takes place, the relevant routing
    /// configuration is made available to those proxies. For each metadataFilter in
    /// this list, if its filterMatchCriteria is set to MATCH_ANY, at least one of the
    /// filterLabels must match the corresponding label provided in the metadata. If its
    /// filterMatchCriteria is set to MATCH_ALL, then all of its filterLabels must match
    /// with corresponding labels in the provided metadata. metadataFilters specified
    /// here can be overrides those specified in ForwardingRule that refers to this
    /// UrlMap. metadataFilters only applies to Loadbalancers that have their
    /// loadBalancingScheme set to INTERNAL_SELF_MANAGED.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "metadataFilters")]
    pub r#metadata_filters: Box<Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleMatchRuleMetadataFilter>>>,
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
    /// For satisfying the matchRule condition, the request's path must begin with the
    /// specified prefixMatch. prefixMatch must begin with a /. The value must be
    /// between 1 and 1024 characters. Only one of prefixMatch, fullPathMatch or
    /// regexMatch must be specified.
    #[builder(into, default)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Box<Option<String>>,
    /// Specifies a list of query parameter match criteria, all of which must match
    /// corresponding query parameters in the request.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "queryParameterMatches")]
    pub r#query_parameter_matches: Box<Option<Vec<super::super::types::compute::UrlMapPathMatcherRouteRuleMatchRuleQueryParameterMatch>>>,
    /// For satisfying the matchRule condition, the path of the request must satisfy the
    /// regular expression specified in regexMatch after removing any query parameters
    /// and anchor supplied with the original URL. For regular expression grammar please
    /// see en.cppreference.com/w/cpp/regex/ecmascript  Only one of prefixMatch,
    /// fullPathMatch or regexMatch must be specified.
    #[builder(into, default)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Box<Option<String>>,
}
