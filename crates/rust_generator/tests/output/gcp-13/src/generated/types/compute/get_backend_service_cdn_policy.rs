#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetBackendServiceCdnPolicy {
    /// Bypass the cache when the specified request headers are matched - e.g. Pragma or Authorization headers. Up to 5 headers can be specified.
    /// The cache is bypassed for all cdnPolicy.cacheMode settings.
    #[builder(into)]
    #[serde(rename = "bypassCacheOnRequestHeaders")]
    pub r#bypass_cache_on_request_headers: Box<Vec<super::super::types::compute::GetBackendServiceCdnPolicyBypassCacheOnRequestHeader>>,
    /// The CacheKeyPolicy for this CdnPolicy.
    #[builder(into)]
    #[serde(rename = "cacheKeyPolicies")]
    pub r#cache_key_policies: Box<Vec<super::super::types::compute::GetBackendServiceCdnPolicyCacheKeyPolicy>>,
    /// Specifies the cache setting for all responses from this backend.
    /// The possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC Possible values: ["USE_ORIGIN_HEADERS", "FORCE_CACHE_ALL", "CACHE_ALL_STATIC"]
    #[builder(into)]
    #[serde(rename = "cacheMode")]
    pub r#cache_mode: Box<String>,
    /// Specifies the maximum allowed TTL for cached content served by this origin.
    #[builder(into)]
    #[serde(rename = "clientTtl")]
    pub r#client_ttl: Box<i32>,
    /// Specifies the default TTL for cached content served by this origin for responses
    /// that do not have an existing valid TTL (max-age or s-max-age).
    #[builder(into)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Box<i32>,
    /// Specifies the maximum allowed TTL for cached content served by this origin.
    #[builder(into)]
    #[serde(rename = "maxTtl")]
    pub r#max_ttl: Box<i32>,
    /// Negative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects.
    #[builder(into)]
    #[serde(rename = "negativeCaching")]
    pub r#negative_caching: Box<bool>,
    /// Sets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.
    /// Omitting the policy and leaving negativeCaching enabled will use Cloud CDN's default cache TTLs.
    #[builder(into)]
    #[serde(rename = "negativeCachingPolicies")]
    pub r#negative_caching_policies: Box<Vec<super::super::types::compute::GetBackendServiceCdnPolicyNegativeCachingPolicy>>,
    /// Serve existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache.
    #[builder(into)]
    #[serde(rename = "serveWhileStale")]
    pub r#serve_while_stale: Box<i32>,
    /// Maximum number of seconds the response to a signed URL request
    /// will be considered fresh, defaults to 1hr (3600s). After this
    /// time period, the response will be revalidated before
    /// being served.
    /// 
    /// When serving responses to signed URL requests, Cloud CDN will
    /// internally behave as though all responses from this backend had a
    /// "Cache-Control: public, max-age=[TTL]" header, regardless of any
    /// existing Cache-Control header. The actual headers served in
    /// responses will not be altered.
    #[builder(into)]
    #[serde(rename = "signedUrlCacheMaxAgeSec")]
    pub r#signed_url_cache_max_age_sec: Box<i32>,
}
