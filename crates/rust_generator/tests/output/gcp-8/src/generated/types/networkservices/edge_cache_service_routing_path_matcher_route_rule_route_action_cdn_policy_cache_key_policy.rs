#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EdgeCacheServiceRoutingPathMatcherRouteRuleRouteActionCdnPolicyCacheKeyPolicy {
    /// If true, requests to different hosts will be cached separately.
    /// Note: this should only be enabled if hosts share the same origin and content. Removing the host from the cache key may inadvertently result in different objects being cached than intended, depending on which route the first user matched.
    #[builder(into, default)]
    #[serde(rename = "excludeHost")]
    pub r#exclude_host: Box<Option<bool>>,
    /// If true, exclude query string parameters from the cache key
    /// If false (the default), include the query string parameters in
    /// the cache key according to includeQueryParameters and
    /// excludeQueryParameters. If neither includeQueryParameters nor
    /// excludeQueryParameters is set, the entire query string will be
    /// included.
    #[builder(into, default)]
    #[serde(rename = "excludeQueryString")]
    pub r#exclude_query_string: Box<Option<bool>>,
    /// Names of query string parameters to exclude from cache keys. All other parameters will be included.
    /// Either specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters.
    #[builder(into, default)]
    #[serde(rename = "excludedQueryParameters")]
    pub r#excluded_query_parameters: Box<Option<Vec<String>>>,
    /// If true, http and https requests will be cached separately.
    #[builder(into, default)]
    #[serde(rename = "includeProtocol")]
    pub r#include_protocol: Box<Option<bool>>,
    /// Names of Cookies to include in cache keys.  The cookie name and cookie value of each cookie named will be used as part of the cache key.
    /// Cookie names:
    /// - must be valid RFC 6265 "cookie-name" tokens
    /// - are case sensitive
    /// - cannot start with "Edge-Cache-" (case insensitive)
    /// Note that specifying several cookies, and/or cookies that have a large range of values (e.g., per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.
    /// You may specify up to three cookie names.
    #[builder(into, default)]
    #[serde(rename = "includedCookieNames")]
    pub r#included_cookie_names: Box<Option<Vec<String>>>,
    /// Names of HTTP request headers to include in cache keys. The value of the header field will be used as part of the cache key.
    /// - Header names must be valid HTTP RFC 7230 header field values.
    /// - Header field names are case insensitive
    /// - To include the HTTP method, use ":method"
    /// Note that specifying several headers, and/or headers that have a large range of values (e.g. per-user) will dramatically impact the cache hit rate, and may result in a higher eviction rate and reduced performance.
    #[builder(into, default)]
    #[serde(rename = "includedHeaderNames")]
    pub r#included_header_names: Box<Option<Vec<String>>>,
    /// Names of query string parameters to include in cache keys. All other parameters will be excluded.
    /// Either specify includedQueryParameters or excludedQueryParameters, not both. '&' and '=' will be percent encoded and not treated as delimiters.
    #[builder(into, default)]
    #[serde(rename = "includedQueryParameters")]
    pub r#included_query_parameters: Box<Option<Vec<String>>>,
}
