#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableMaintenanceConfigurationIcebergCompaction {
    /// Settings for compaction.
    /// See `iceberg_compaction.settings` below
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Box<super::super::types::s3tables::TableMaintenanceConfigurationIcebergCompactionSettings>,
    /// Whether the configuration is enabled.
    /// Valid values are `enabled` and `disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
