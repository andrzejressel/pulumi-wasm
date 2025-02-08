#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsWebAppLogApplicationLog {
    /// A `azure_blob_storage` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureBlobStorages")]
    pub r#azure_blob_storages: Box<Vec<super::super::types::appservice::GetWindowsWebAppLogApplicationLogAzureBlobStorage>>,
    /// The logging level.
    #[builder(into)]
    #[serde(rename = "fileSystemLevel")]
    pub r#file_system_level: Box<String>,
}
