#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MethodSettingsSettings {
    /// Whether the cached responses are encrypted.
    #[builder(into, default)]
    #[serde(rename = "cacheDataEncrypted")]
    pub r#cache_data_encrypted: Box<Option<bool>>,
    /// Time to live (TTL), in seconds, for cached responses. The higher the TTL, the longer the response will be cached.
    #[builder(into, default)]
    #[serde(rename = "cacheTtlInSeconds")]
    pub r#cache_ttl_in_seconds: Box<Option<i32>>,
    /// Whether responses should be cached and returned for requests. A cache cluster must be enabled on the stage for responses to be cached.
    #[builder(into, default)]
    #[serde(rename = "cachingEnabled")]
    pub r#caching_enabled: Box<Option<bool>>,
    /// Whether data trace logging is enabled for this method, which effects the log entries pushed to Amazon CloudWatch Logs.
    #[builder(into, default)]
    #[serde(rename = "dataTraceEnabled")]
    pub r#data_trace_enabled: Box<Option<bool>>,
    /// Logging level for this method, which effects the log entries pushed to Amazon CloudWatch Logs. The available levels are `OFF`, `ERROR`, and `INFO`.
    #[builder(into, default)]
    #[serde(rename = "loggingLevel")]
    pub r#logging_level: Box<Option<String>>,
    /// Whether Amazon CloudWatch metrics are enabled for this method.
    #[builder(into, default)]
    #[serde(rename = "metricsEnabled")]
    pub r#metrics_enabled: Box<Option<bool>>,
    /// Whether authorization is required for a cache invalidation request.
    #[builder(into, default)]
    #[serde(rename = "requireAuthorizationForCacheControl")]
    pub r#require_authorization_for_cache_control: Box<Option<bool>>,
    /// Throttling burst limit. Default: `-1` (throttling disabled).
    #[builder(into, default)]
    #[serde(rename = "throttlingBurstLimit")]
    pub r#throttling_burst_limit: Box<Option<i32>>,
    /// Throttling rate limit. Default: `-1` (throttling disabled).
    #[builder(into, default)]
    #[serde(rename = "throttlingRateLimit")]
    pub r#throttling_rate_limit: Box<Option<f64>>,
    /// How to handle unauthorized requests for cache invalidation. The available values are `FAIL_WITH_403`, `SUCCEED_WITH_RESPONSE_HEADER`, `SUCCEED_WITHOUT_RESPONSE_HEADER`.
    #[builder(into, default)]
    #[serde(rename = "unauthorizedCacheControlHeaderStrategy")]
    pub r#unauthorized_cache_control_header_strategy: Box<Option<String>>,
}
