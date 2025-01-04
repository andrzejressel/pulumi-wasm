#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerRuleCondition {
    /// Contains a single `values` item which is a list of host header patterns to match. The maximum size of each pattern is 128 characters. Comparison is case insensitive. Wildcard characters supported: * (matches 0 or more characters) and ? (matches exactly 1 character). Only one pattern needs to match for the condition to be satisfied.
    #[builder(into, default)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<super::super::types::alb::ListenerRuleConditionHostHeader>>,
    /// HTTP headers to match. HTTP Header block fields documented below.
    #[builder(into, default)]
    #[serde(rename = "httpHeader")]
    pub r#http_header: Box<Option<super::super::types::alb::ListenerRuleConditionHttpHeader>>,
    /// Contains a single `values` item which is a list of HTTP request methods or verbs to match. Maximum size is 40 characters. Only allowed characters are A-Z, hyphen (-) and underscore (\_). Comparison is case sensitive. Wildcards are not supported. Only one needs to match for the condition to be satisfied. AWS recommends that GET and HEAD requests are routed in the same way because the response to a HEAD request may be cached.
    #[builder(into, default)]
    #[serde(rename = "httpRequestMethod")]
    pub r#http_request_method: Box<Option<super::super::types::alb::ListenerRuleConditionHttpRequestMethod>>,
    /// Contains a single `values` item which is a list of path patterns to match against the request URL. Maximum size of each pattern is 128 characters. Comparison is case sensitive. Wildcard characters supported: * (matches 0 or more characters) and ? (matches exactly 1 character). Only one pattern needs to match for the condition to be satisfied. Path pattern is compared only to the path of the URL, not to its query string. To compare against the query string, use a `query_string` condition.
    #[builder(into, default)]
    #[serde(rename = "pathPattern")]
    pub r#path_pattern: Box<Option<super::super::types::alb::ListenerRuleConditionPathPattern>>,
    /// Query strings to match. Query String block fields documented below.
    #[builder(into, default)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Box<Option<Vec<super::super::types::alb::ListenerRuleConditionQueryString>>>,
    /// Contains a single `values` item which is a list of source IP CIDR notations to match. You can use both IPv4 and IPv6 addresses. Wildcards are not supported. Condition is satisfied if the source IP address of the request matches one of the CIDR blocks. Condition is not satisfied by the addresses in the `X-Forwarded-For` header, use `http_header` condition instead.
    /// 
    /// > **NOTE::** Exactly one of `host_header`, `http_header`, `http_request_method`, `path_pattern`, `query_string` or `source_ip` must be set per condition.
    #[builder(into, default)]
    #[serde(rename = "sourceIp")]
    pub r#source_ip: Box<Option<super::super::types::alb::ListenerRuleConditionSourceIp>>,
}
