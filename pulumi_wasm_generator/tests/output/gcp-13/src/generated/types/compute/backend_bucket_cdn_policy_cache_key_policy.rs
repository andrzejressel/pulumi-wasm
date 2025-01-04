#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BackendBucketCdnPolicyCacheKeyPolicy {
    /// Allows HTTP request headers (by name) to be used in the
    /// cache key.
    #[builder(into, default)]
    #[serde(rename = "includeHttpHeaders")]
    pub r#include_http_headers: Box<Option<Vec<String>>>,
    /// Names of query string parameters to include in cache keys.
    /// Default parameters are always included. '&' and '=' will
    /// be percent encoded and not treated as delimiters.
    #[builder(into, default)]
    #[serde(rename = "queryStringWhitelists")]
    pub r#query_string_whitelists: Box<Option<Vec<String>>>,
}
