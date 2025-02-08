#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval {
    /// Settings for unreferenced file removal.
    /// See `iceberg_unreferenced_file_removal.settings` below
    #[builder(into)]
    #[serde(rename = "settings")]
    pub r#settings: Box<super::super::types::s3tables::TableBucketMaintenanceConfigurationIcebergUnreferencedFileRemovalSettings>,
    /// Whether the configuration is enabled.
    /// Valid values are `enabled` and `disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
}
