#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig {
    /// Whether URL query strings in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values for `query_string_behavior` are `none`, `whitelist`, `allExcept`, and `all`.
    #[builder(into)]
    #[serde(rename = "queryStringBehavior")]
    pub r#query_string_behavior: Box<String>,
    /// Configuration parameter that contains a list of query string names. See Items for more information.
    #[builder(into, default)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Box<Option<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfigQueryStrings>>,
}
