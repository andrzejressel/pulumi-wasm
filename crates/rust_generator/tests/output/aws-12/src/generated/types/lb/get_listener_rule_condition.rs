#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerRuleCondition {
    /// Contains a single attribute `values`, which contains a set of host names.
    #[builder(into, default)]
    #[serde(rename = "hostHeader")]
    pub r#host_header: Box<Option<super::super::types::lb::GetListenerRuleConditionHostHeader>>,
    /// HTTP header and values to match.
    /// Detailed below.
    #[builder(into, default)]
    #[serde(rename = "httpHeader")]
    pub r#http_header: Box<Option<super::super::types::lb::GetListenerRuleConditionHttpHeader>>,
    /// Contains a single attribute `values`, which contains a set of HTTP request methods.
    #[builder(into, default)]
    #[serde(rename = "httpRequestMethod")]
    pub r#http_request_method: Box<Option<super::super::types::lb::GetListenerRuleConditionHttpRequestMethod>>,
    /// Contains a single attribute `values`, which contains a set of path patterns to compare against the request URL.
    #[builder(into, default)]
    #[serde(rename = "pathPattern")]
    pub r#path_pattern: Box<Option<super::super::types::lb::GetListenerRuleConditionPathPattern>>,
    /// Query string parameters to match.
    /// Detailed below.
    #[builder(into, default)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<super::super::types::lb::GetListenerRuleConditionQueryString>>,
    /// Contains a single attribute `values`, which contains a set of source IPs in CIDR notation.
    #[builder(into, default)]
    #[serde(rename = "sourceIp")]
    pub r#source_ip: Box<Option<super::super::types::lb::GetListenerRuleConditionSourceIp>>,
}
