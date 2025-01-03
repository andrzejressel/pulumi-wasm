#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct StreamSourceConfigOracleSourceConfig {
    /// Configuration to drop large object values.
    #[builder(into, default)]
    #[serde(rename = "dropLargeObjects")]
    pub r#drop_large_objects: Box<Option<super::super::types::datastream::StreamSourceConfigOracleSourceConfigDropLargeObjects>>,
    /// Oracle objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludeObjects")]
    pub r#exclude_objects: Box<Option<super::super::types::datastream::StreamSourceConfigOracleSourceConfigExcludeObjects>>,
    /// Oracle objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "includeObjects")]
    pub r#include_objects: Box<Option<super::super::types::datastream::StreamSourceConfigOracleSourceConfigIncludeObjects>>,
    /// Maximum number of concurrent backfill tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentBackfillTasks")]
    pub r#max_concurrent_backfill_tasks: Box<Option<i32>>,
    /// Maximum number of concurrent CDC tasks. The number should be non negative.
    /// If not set (or set to 0), the system's default value will be used.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentCdcTasks")]
    pub r#max_concurrent_cdc_tasks: Box<Option<i32>>,
    /// Configuration to drop large object values.
    #[builder(into, default)]
    #[serde(rename = "streamLargeObjects")]
    pub r#stream_large_objects: Box<Option<super::super::types::datastream::StreamSourceConfigOracleSourceConfigStreamLargeObjects>>,
}
