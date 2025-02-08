#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataRepositoryAssociationS3 {
    /// Specifies the type of updated objects that will be automatically exported from your file system to the linked S3 bucket. See the `events` configuration block.
    #[builder(into, default)]
    #[serde(rename = "autoExportPolicy")]
    pub r#auto_export_policy: Box<Option<super::super::types::fsx::DataRepositoryAssociationS3AutoExportPolicy>>,
    /// Specifies the type of updated objects that will be automatically imported from the linked S3 bucket to your file system. See the `events` configuration block.
    #[builder(into, default)]
    #[serde(rename = "autoImportPolicy")]
    pub r#auto_import_policy: Box<Option<super::super::types::fsx::DataRepositoryAssociationS3AutoImportPolicy>>,
}
