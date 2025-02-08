#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableBucketMaintenanceConfiguration {
    /// A single Iceberg unreferenced file removal settings block.
    /// See `iceberg_unreferenced_file_removal` below
    #[builder(into)]
    #[serde(rename = "icebergUnreferencedFileRemoval")]
    pub r#iceberg_unreferenced_file_removal: Box<super::super::types::s3tables::TableBucketMaintenanceConfigurationIcebergUnreferencedFileRemoval>,
}
