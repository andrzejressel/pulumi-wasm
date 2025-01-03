#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RegionBackendServiceCdnPolicy {
    /// The CacheKeyPolicy for this CdnPolicy.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "cacheKeyPolicy")]
    pub r#cache_key_policy: Box<Option<super::super::types::compute::RegionBackendServiceCdnPolicyCacheKeyPolicy>>,
    /// Specifies the cache setting for all responses from this backend.
    /// The possible values are: USE_ORIGIN_HEADERS, FORCE_CACHE_ALL and CACHE_ALL_STATIC
    /// Possible values are: `USE_ORIGIN_HEADERS`, `FORCE_CACHE_ALL`, `CACHE_ALL_STATIC`.
    #[builder(into, default)]
    #[serde(rename = "cacheMode")]
    pub r#cache_mode: Box<Option<String>>,
    /// Specifies the maximum allowed TTL for cached content served by this origin.
    #[builder(into, default)]
    #[serde(rename = "clientTtl")]
    pub r#client_ttl: Box<Option<i32>>,
    /// Specifies the default TTL for cached content served by this origin for responses
    /// that do not have an existing valid TTL (max-age or s-max-age).
    #[builder(into, default)]
    #[serde(rename = "defaultTtl")]
    pub r#default_ttl: Box<Option<i32>>,
    /// Specifies the maximum allowed TTL for cached content served by this origin.
    #[builder(into, default)]
    #[serde(rename = "maxTtl")]
    pub r#max_ttl: Box<Option<i32>>,
    /// Negative caching allows per-status code TTLs to be set, in order to apply fine-grained caching for common errors or redirects.
    #[builder(into, default)]
    #[serde(rename = "negativeCaching")]
    pub r#negative_caching: Box<Option<bool>>,
    /// Sets a cache TTL for the specified HTTP status code. negativeCaching must be enabled to configure negativeCachingPolicy.
    /// Omitting the policy and leaving negativeCaching enabled will use Cloud CDN's default cache TTLs.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "negativeCachingPolicies")]
    pub r#negative_caching_policies: Box<Option<Vec<super::super::types::compute::RegionBackendServiceCdnPolicyNegativeCachingPolicy>>>,
    /// Serve existing content from the cache (if available) when revalidating content with the origin, or when an error is encountered when refreshing the cache.
    #[builder(into, default)]
    #[serde(rename = "serveWhileStale")]
    pub r#serve_while_stale: Box<Option<i32>>,
    /// Maximum number of seconds the response to a signed URL request
    /// will be considered fresh, defaults to 1hr (3600s). After this
    /// time period, the response will be revalidated before
    /// being served.
    /// When serving responses to signed URL requests, Cloud CDN will
    /// internally behave as though all responses from this backend had a
    /// "Cache-Control: public, max-age=[TTL]" header, regardless of any
    /// existing Cache-Control header. The actual headers served in
    /// responses will not be altered.
    #[builder(into, default)]
    #[serde(rename = "signedUrlCacheMaxAgeSec")]
    pub r#signed_url_cache_max_age_sec: Box<Option<i32>>,
}
