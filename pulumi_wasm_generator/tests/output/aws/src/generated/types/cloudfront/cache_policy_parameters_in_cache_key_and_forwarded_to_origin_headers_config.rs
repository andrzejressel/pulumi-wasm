#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig {
    /// Whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values for `header_behavior` are `none` and `whitelist`.
    #[builder(into, default)]
    #[serde(rename = "headerBehavior")]
    pub r#header_behavior: Box<Option<String>>,
    /// Object contains a list of header names. See Items for more information.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeaders>>,
}
