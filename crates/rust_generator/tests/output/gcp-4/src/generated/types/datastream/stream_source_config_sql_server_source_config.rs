#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct StreamSourceConfigSqlServerSourceConfig {
    /// CDC reader reads from change tables.
    #[builder(into, default)]
    #[serde(rename = "changeTables")]
    pub r#change_tables: Box<Option<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigChangeTables>>,
    /// SQL Server objects to exclude from the stream.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "excludeObjects")]
    pub r#exclude_objects: Box<Option<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigExcludeObjects>>,
    /// SQL Server objects to retrieve from the source.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "includeObjects")]
    pub r#include_objects: Box<Option<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigIncludeObjects>>,
    /// Max concurrent backfill tasks.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentBackfillTasks")]
    pub r#max_concurrent_backfill_tasks: Box<Option<i32>>,
    /// Max concurrent CDC tasks.
    #[builder(into, default)]
    #[serde(rename = "maxConcurrentCdcTasks")]
    pub r#max_concurrent_cdc_tasks: Box<Option<i32>>,
    /// CDC reader reads from transaction logs.
    #[builder(into, default)]
    #[serde(rename = "transactionLogs")]
    pub r#transaction_logs: Box<Option<super::super::types::datastream::StreamSourceConfigSqlServerSourceConfigTransactionLogs>>,
}
