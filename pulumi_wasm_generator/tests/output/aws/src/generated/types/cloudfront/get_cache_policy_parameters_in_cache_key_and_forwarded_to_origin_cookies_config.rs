#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfig {
    /// Determines whether any cookies in viewer requests are included in the cache key and automatically included in requests that CloudFront sends to the origin. Valid values are `none`, `whitelist`, `allExcept`, `all`.
    #[builder(into)]
    #[serde(rename = "cookieBehavior")]
    pub r#cookie_behavior: Box<String>,
    /// Object that contains a list of cookie names. See Items for more information.
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Box<Vec<super::super::types::cloudfront::GetCachePolicyParametersInCacheKeyAndForwardedToOriginCookiesConfigCookie>>,
}