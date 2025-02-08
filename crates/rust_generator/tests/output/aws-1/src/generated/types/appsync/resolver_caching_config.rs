#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ResolverCachingConfig {
    /// The caching keys for a resolver that has caching activated. Valid values are entries from the $context.arguments, $context.source, and $context.identity maps.
    #[builder(into, default)]
    #[serde(rename = "cachingKeys")]
    pub r#caching_keys: Box<Option<Vec<String>>>,
    /// The TTL in seconds for a resolver that has caching activated. Valid values are between `1` and `3600` seconds.
    #[builder(into, default)]
    #[serde(rename = "ttl")]
    pub r#ttl: Box<Option<i32>>,
}
