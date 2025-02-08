#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DistributionOrderedCacheBehaviorForwardedValues {
    /// The forwarded values cookies that specifies how CloudFront handles cookies (maximum one).
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Box<super::super::types::cloudfront::DistributionOrderedCacheBehaviorForwardedValuesCookies>,
    /// Headers, if any, that you want CloudFront to vary upon for this cache behavior. Specify `*` to include all headers.
    #[builder(into, default)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Option<Vec<String>>>,
    /// Indicates whether you want CloudFront to forward query strings to the origin that is associated with this cache behavior.
    #[builder(into)]
    #[serde(rename = "queryString")]
    pub r#query_string: Box<bool>,
    /// When specified, along with a value of `true` for `query_string`, all query strings are forwarded, however only the query string keys listed in this argument are cached. When omitted with a value of `true` for `query_string`, all query string keys are cached.
    #[builder(into, default)]
    #[serde(rename = "queryStringCacheKeys")]
    pub r#query_string_cache_keys: Box<Option<Vec<String>>>,
}
