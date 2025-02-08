#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableTableReplicationInfo {
    /// The interval at which the source
    /// materialized view is polled for updates. The default is 300000.
    #[builder(into, default)]
    #[serde(rename = "replicationIntervalMs")]
    pub r#replication_interval_ms: Box<Option<i32>>,
    /// The ID of the source dataset.
    #[builder(into)]
    #[serde(rename = "sourceDatasetId")]
    pub r#source_dataset_id: Box<String>,
    /// The ID of the source project.
    #[builder(into)]
    #[serde(rename = "sourceProjectId")]
    pub r#source_project_id: Box<String>,
    /// The ID of the source materialized view.
    #[builder(into)]
    #[serde(rename = "sourceTableId")]
    pub r#source_table_id: Box<String>,
}
