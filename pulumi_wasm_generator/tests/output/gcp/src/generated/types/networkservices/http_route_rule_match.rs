#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct HttpRouteRuleMatch {
    /// The HTTP request path value should exactly match this value.
    #[builder(into, default)]
    #[serde(rename = "fullPathMatch")]
    pub r#full_path_match: Box<Option<String>>,
    /// Specifies a list of HTTP request headers to match against.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::networkservices::HttpRouteRuleMatchHeader>>>,
    /// Specifies if prefixMatch and fullPathMatch matches are case sensitive. The default value is false.
    #[builder(into, default)]
    #[serde(rename = "ignoreCase")]
    pub r#ignore_case: Box<Option<bool>>,
    /// The HTTP request path value must begin with specified prefixMatch. prefixMatch must begin with a /.
    #[builder(into, default)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Box<Option<String>>,
    /// Specifies a list of query parameters to match against.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "queryParameters")]
    pub r#query_parameters: Box<Option<Vec<super::super::types::networkservices::HttpRouteRuleMatchQueryParameter>>>,
    /// The HTTP request path value must satisfy the regular expression specified by regexMatch after removing any query parameters and anchor supplied with the original URL. For regular expression grammar, please see https://github.com/google/re2/wiki/Syntax
    #[builder(into, default)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Box<Option<String>>,
}
