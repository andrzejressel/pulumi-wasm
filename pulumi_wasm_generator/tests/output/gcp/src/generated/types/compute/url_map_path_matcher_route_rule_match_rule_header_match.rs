#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UrlMapPathMatcherRouteRuleMatchRuleHeaderMatch {
    /// The value should exactly match contents of exactMatch. Only one of exactMatch,
    /// prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[builder(into, default)]
    #[serde(rename = "exactMatch")]
    pub r#exact_match: Box<Option<String>>,
    /// The name of the HTTP header to match. For matching against the HTTP request's
    /// authority, use a headerMatch with the header name ":authority". For matching a
    /// request's method, use the headerName ":method".
    #[builder(into)]
    #[serde(rename = "headerName")]
    pub r#header_name: Box<String>,
    /// If set to false, the headerMatch is considered a match if the match criteria
    /// above are met. If set to true, the headerMatch is considered a match if the
    /// match criteria above are NOT met. Defaults to false.
    #[builder(into, default)]
    #[serde(rename = "invertMatch")]
    pub r#invert_match: Box<Option<bool>>,
    /// The value of the header must start with the contents of prefixMatch. Only one of
    /// exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch
    /// must be set.
    #[builder(into, default)]
    #[serde(rename = "prefixMatch")]
    pub r#prefix_match: Box<Option<String>>,
    /// A header with the contents of headerName must exist. The match takes place
    /// whether or not the request's header has a value or not. Only one of exactMatch,
    /// prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch must be set.
    #[builder(into, default)]
    #[serde(rename = "presentMatch")]
    pub r#present_match: Box<Option<bool>>,
    /// The header value must be an integer and its value must be in the range specified
    /// in rangeMatch. If the header does not contain an integer, number or is empty,
    /// the match fails. For example for a range [-5, 0]   - -3 will match.  - 0 will
    /// not match.  - 0.25 will not match.  - -3someString will not match.   Only one of
    /// exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch
    /// must be set.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "rangeMatch")]
    pub r#range_match: Box<Option<super::super::types::compute::UrlMapPathMatcherRouteRuleMatchRuleHeaderMatchRangeMatch>>,
    /// The value of the header must match the regular expression specified in
    /// regexMatch. For regular expression grammar, please see:
    /// en.cppreference.com/w/cpp/regex/ecmascript  For matching against a port
    /// specified in the HTTP request, use a headerMatch with headerName set to PORT and
    /// a regular expression that satisfies the RFC2616 Host header's port specifier.
    /// Only one of exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or
    /// rangeMatch must be set.
    #[builder(into, default)]
    #[serde(rename = "regexMatch")]
    pub r#regex_match: Box<Option<String>>,
    /// The value of the header must end with the contents of suffixMatch. Only one of
    /// exactMatch, prefixMatch, suffixMatch, regexMatch, presentMatch or rangeMatch
    /// must be set.
    #[builder(into, default)]
    #[serde(rename = "suffixMatch")]
    pub r#suffix_match: Box<Option<String>>,
}
