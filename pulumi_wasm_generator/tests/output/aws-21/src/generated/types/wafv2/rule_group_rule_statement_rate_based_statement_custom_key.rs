#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleGroupRuleStatementRateBasedStatementCustomKey {
    /// (Optional) Use the value of a cookie in the request as an aggregate key. See RateLimit `cookie` below for details.
    #[builder(into, default)]
    #[serde(rename = "cookie")]
    pub r#cookie: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyCookie>>,
    /// (Optional) Use the first IP address in an HTTP header as an aggregate key. See `forwarded_ip` below for details.
    #[builder(into, default)]
    #[serde(rename = "forwardedIp")]
    pub r#forwarded_ip: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyForwardedIp>>,
    /// (Optional) Use the value of a header in the request as an aggregate key. See RateLimit `header` below for details.
    #[builder(into, default)]
    #[serde(rename = "header")]
    pub r#header: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyHeader>>,
    /// (Optional) Use the request's HTTP method as an aggregate key. See RateLimit `http_method` below for details.
    #[builder(into, default)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyHttpMethod>>,
    /// (Optional) Use the request's originating IP address as an aggregate key. See `RateLimit ip` below for details.
    #[builder(into, default)]
    #[serde(rename = "ip")]
    pub r#ip: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyIp>>,
    /// (Optional) Use the specified label namespace as an aggregate key. See RateLimit `label_namespace` below for details.
    #[builder(into, default)]
    #[serde(rename = "labelNamespace")]
    pub r#label_namespace: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyLabelNamespace>>,
    /// (Optional) Use the specified query argument as an aggregate key. See RateLimit `query_argument` below for details.
    #[builder(into, default)]
    #[serde(rename = "queryArgument")]
    pub r#query_argument: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyQueryArgument>>,
    /// (Optional) Use the request's query string as an aggregate key. See RateLimit `query_string` below for details.
    #[builder(into, default)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyQueryString>>,
    /// (Optional) Use the request's URI path as an aggregate key. See RateLimit `uri_path` below for details.
    #[builder(into, default)]
    #[serde(rename = "uriPath")]
    pub r#uri_path: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementCustomKeyUriPath>>,
}
