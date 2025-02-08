#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SlotLogsHttpLogs {
    /// An `azure_blob_storage` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "azureBlobStorage")]
    pub r#azure_blob_storage: Box<Option<super::super::types::appservice::SlotLogsHttpLogsAzureBlobStorage>>,
    /// A `file_system` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "fileSystem")]
    pub r#file_system: Box<Option<super::super::types::appservice::SlotLogsHttpLogsFileSystem>>,
}
