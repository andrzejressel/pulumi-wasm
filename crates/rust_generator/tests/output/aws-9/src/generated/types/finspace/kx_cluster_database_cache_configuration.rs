#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct KxClusterDatabaseCacheConfiguration {
    /// Type of disk cache.
    #[builder(into)]
    #[serde(rename = "cacheType")]
    pub r#cache_type: Box<String>,
    /// Paths within the database to cache.
    #[builder(into, default)]
    #[serde(rename = "dbPaths")]
    pub r#db_paths: Box<Option<Vec<String>>>,
}
