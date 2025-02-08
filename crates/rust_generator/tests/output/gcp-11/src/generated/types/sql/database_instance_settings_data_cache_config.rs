#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DatabaseInstanceSettingsDataCacheConfig {
    /// Whether data cache is enabled for the instance. Defaults to `false`. Can be used with MYSQL and PostgreSQL only.
    #[builder(into, default)]
    #[serde(rename = "dataCacheEnabled")]
    pub r#data_cache_enabled: Box<Option<bool>>,
}
