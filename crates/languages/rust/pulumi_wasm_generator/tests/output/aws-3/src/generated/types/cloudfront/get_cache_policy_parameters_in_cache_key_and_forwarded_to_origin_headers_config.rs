#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig {
    /// Determines whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are `none`, `whitelist`.
    #[builder(into)]
    #[serde(rename = "headerBehavior")]
    pub r#header_behavior: Box<String>,
    /// Object that contains a list of header names. See Items for more information.
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfigHeader>>,
}
