#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOrigin {
    /// Object that determines whether any cookies in viewer requests (and if so, which cookies) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
    #[builder(into)]
    #[serde(rename = "cookiesConfigs")]
    pub r#cookies_configs: Box<Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig>>,
    /// A flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
    #[builder(into)]
    #[serde(rename = "enableAcceptEncodingBrotli")]
    pub r#enable_accept_encoding_brotli: Box<bool>,
    /// A flag that can affect whether the Accept-Encoding HTTP header is included in the cache key and included in requests that CloudFront sends to the origin.
    #[builder(into)]
    #[serde(rename = "enableAcceptEncodingGzip")]
    pub r#enable_accept_encoding_gzip: Box<bool>,
    /// Object that determines whether any HTTP headers (and if so, which headers) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
    #[builder(into)]
    #[serde(rename = "headersConfigs")]
    pub r#headers_configs: Box<Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig>>,
    /// Object that determines whether any URL query strings in viewer requests (and if so, which query strings) are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Query String Config for more information.
    #[builder(into)]
    #[serde(rename = "queryStringsConfigs")]
    pub r#query_strings_configs: Box<Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig>>,
}
