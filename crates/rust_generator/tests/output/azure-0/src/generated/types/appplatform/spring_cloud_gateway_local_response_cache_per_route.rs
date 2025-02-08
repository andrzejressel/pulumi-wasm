#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SpringCloudGatewayLocalResponseCachePerRoute {
    /// Specifies the maximum size of cache (10MB, 900KB, 1GB...) to determine if the cache needs to evict some entries.
    #[builder(into, default)]
    #[serde(rename = "size")]
    pub r#size: Box<Option<String>>,
    /// Specifies the time before a cached entry is expired (300s, 5m, 1h...).
    #[builder(into, default)]
    #[serde(rename = "timeToLive")]
    pub r#time_to_live: Box<Option<String>>,
}
