#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackendServiceCdnPolicyCacheKeyPolicy {
    /// If true requests to different hosts will be cached separately.
    #[builder(into)]
    #[serde(rename = "includeHost")]
    pub r#include_host: Box<bool>,
    /// Allows HTTP request headers (by name) to be used in the
    /// cache key.
    #[builder(into)]
    #[serde(rename = "includeHttpHeaders")]
    pub r#include_http_headers: Box<Vec<String>>,
    /// Names of cookies to include in cache keys.
    #[builder(into)]
    #[serde(rename = "includeNamedCookies")]
    pub r#include_named_cookies: Box<Vec<String>>,
    /// If true, http and https requests will be cached separately.
    #[builder(into)]
    #[serde(rename = "includeProtocol")]
    pub r#include_protocol: Box<bool>,
    /// If true, include query string parameters in the cache key
    /// according to query_string_whitelist and
    /// query_string_blacklist. If neither is set, the entire query
    /// string will be included.
    /// 
    /// If false, the query string will be excluded from the cache
    /// key entirely.
    #[builder(into)]
    #[serde(rename = "includeQueryString")]
    pub r#include_query_string: Box<bool>,
    /// Names of query string parameters to exclude in cache keys.
    /// 
    /// All other parameters will be included. Either specify
    /// query_string_whitelist or query_string_blacklist, not both.
    /// '&' and '=' will be percent encoded and not treated as
    /// delimiters.
    #[builder(into)]
    #[serde(rename = "queryStringBlacklists")]
    pub r#query_string_blacklists: Box<Vec<String>>,
    /// Names of query string parameters to include in cache keys.
    /// 
    /// All other parameters will be excluded. Either specify
    /// query_string_whitelist or query_string_blacklist, not both.
    /// '&' and '=' will be percent encoded and not treated as
    /// delimiters.
    #[builder(into)]
    #[serde(rename = "queryStringWhitelists")]
    pub r#query_string_whitelists: Box<Vec<String>>,
}
