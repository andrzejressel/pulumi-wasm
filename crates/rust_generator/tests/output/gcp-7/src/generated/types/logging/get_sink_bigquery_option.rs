#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetSinkBigqueryOption {
    /// Whether [BigQuery's partition tables](https://cloud.google.com/bigquery/docs/partitioned-tables) are used.
    #[builder(into)]
    #[serde(rename = "usePartitionedTables")]
    pub r#use_partitioned_tables: Box<bool>,
}
