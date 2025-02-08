#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackendBucketCdnPolicyCacheKeyPolicy {
    /// Allows HTTP request headers (by name) to be used in the
    /// cache key.
    #[builder(into)]
    #[serde(rename = "includeHttpHeaders")]
    pub r#include_http_headers: Box<Vec<String>>,
    /// Names of query string parameters to include in cache keys.
    /// Default parameters are always included. '&' and '=' will
    /// be percent encoded and not treated as delimiters.
    #[builder(into)]
    #[serde(rename = "queryStringWhitelists")]
    pub r#query_string_whitelists: Box<Vec<String>>,
}
