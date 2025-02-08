#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct CachePolicyParametersInCacheKeyAndForwardedToOrigin {
    /// Whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Cookies Config for more information.
    #[builder(into)]
    #[serde(rename = "cookiesConfig")]
    pub r#cookies_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig>,
    /// Flag determines whether the Accept-Encoding HTTP header is included in the cache key and in requests that CloudFront sends to the origin.
    #[builder(into, default)]
    #[serde(rename = "enableAcceptEncodingBrotli")]
    pub r#enable_accept_encoding_brotli: Box<Option<bool>>,
    /// Whether the Accept-Encoding HTTP header is included in the cache key and in requests sent to the origin by CloudFront.
    #[builder(into, default)]
    #[serde(rename = "enableAcceptEncodingGzip")]
    pub r#enable_accept_encoding_gzip: Box<Option<bool>>,
    /// Whether any HTTP headers are included in the cache key and automatically included in requests that CloudFront sends to the origin. See Headers Config for more information.
    #[builder(into)]
    #[serde(rename = "headersConfig")]
    pub r#headers_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginHeadersConfig>,
    /// Whether any URL query strings in viewer requests are included in the cache key. It also automatically includes these query strings in requests that CloudFront sends to the origin. Please refer to the Query String Config for more information.
    #[builder(into)]
    #[serde(rename = "queryStringsConfig")]
    pub r#query_strings_config: Box<super::super::types::cloudfront::CachePolicyParametersInCacheKeyAndForwardedToOriginQueryStringsConfig>,
}
