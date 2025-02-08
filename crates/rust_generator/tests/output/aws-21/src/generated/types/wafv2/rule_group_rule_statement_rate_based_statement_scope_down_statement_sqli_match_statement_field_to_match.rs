#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatch {
    /// Inspect all query arguments.
    #[builder(into, default)]
    #[serde(rename = "allQueryArguments")]
    pub r#all_query_arguments: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchAllQueryArguments>>,
    /// Inspect the request body, which immediately follows the request headers.
    #[builder(into, default)]
    #[serde(rename = "body")]
    pub r#body: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchBody>>,
    /// Inspect the cookies in the web request. See Cookies below for details.
    #[builder(into, default)]
    #[serde(rename = "cookies")]
    pub r#cookies: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchCookies>>,
    /// Inspect the request headers. See Header Order below for details.
    #[builder(into, default)]
    #[serde(rename = "headerOrders")]
    pub r#header_orders: Box<Option<Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchHeaderOrder>>>,
    /// Inspect the request headers. See Headers below for details.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchHeader>>>,
    #[builder(into, default)]
    #[serde(rename = "ja3Fingerprint")]
    pub r#ja_3_fingerprint: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchJa3Fingerprint>>,
    /// Inspect the request body as JSON. See JSON Body for details.
    #[builder(into, default)]
    #[serde(rename = "jsonBody")]
    pub r#json_body: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchJsonBody>>,
    /// Inspect the HTTP method. The method indicates the type of operation that the request is asking the origin to perform.
    #[builder(into, default)]
    #[serde(rename = "method")]
    pub r#method: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchMethod>>,
    /// Inspect the query string. This is the part of a URL that appears after a `?` character, if any.
    #[builder(into, default)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchQueryString>>,
    /// Inspect a single header. See Single Header below for details.
    #[builder(into, default)]
    #[serde(rename = "singleHeader")]
    pub r#single_header: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchSingleHeader>>,
    /// Inspect a single query argument. See Single Query Argument below for details.
    #[builder(into, default)]
    #[serde(rename = "singleQueryArgument")]
    pub r#single_query_argument: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchSingleQueryArgument>>,
    /// Inspect the request URI path. This is the part of a web request that identifies a resource, for example, `/images/daily-ad.jpg`.
    #[builder(into, default)]
    #[serde(rename = "uriPath")]
    pub r#uri_path: Box<Option<super::super::types::wafv2::RuleGroupRuleStatementRateBasedStatementScopeDownStatementSqliMatchStatementFieldToMatchUriPath>>,
}
