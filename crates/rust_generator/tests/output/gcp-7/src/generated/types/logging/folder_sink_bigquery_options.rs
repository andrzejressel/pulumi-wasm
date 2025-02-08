#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FolderSinkBigqueryOptions {
    /// Whether to use [BigQuery's partition tables](https://cloud.google.com/bigquery/docs/partitioned-tables).
    /// By default, Logging creates dated tables based on the log entries' timestamps, e.g. syslog_20170523. With partitioned
    /// tables, the date suffix is no longer present and [special query syntax](https://cloud.google.com/bigquery/docs/querying-partitioned-tables)
    /// has to be used instead. In both cases, tables are sharded based on UTC timezone.
    #[builder(into)]
    #[serde(rename = "usePartitionedTables")]
    pub r#use_partitioned_tables: Box<bool>,
}
