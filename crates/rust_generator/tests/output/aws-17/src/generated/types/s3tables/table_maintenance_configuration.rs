#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableMaintenanceConfiguration {
    /// A single Iceberg compaction settings block.
    /// See `iceberg_compaction` below
    #[builder(into)]
    #[serde(rename = "icebergCompaction")]
    pub r#iceberg_compaction: Box<super::super::types::s3tables::TableMaintenanceConfigurationIcebergCompaction>,
    /// A single Iceberg snapshot management settings block.
    /// See `iceberg_snapshot_management` below
    #[builder(into)]
    #[serde(rename = "icebergSnapshotManagement")]
    pub r#iceberg_snapshot_management: Box<super::super::types::s3tables::TableMaintenanceConfigurationIcebergSnapshotManagement>,
}
