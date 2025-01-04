#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSlotLogsHttpLogs {
    /// A `azure_blob_storage_http` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "azureBlobStorage")]
    pub r#azure_blob_storage: Box<Option<super::super::types::appservice::WindowsWebAppSlotLogsHttpLogsAzureBlobStorage>>,
    /// A `file_system` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "fileSystem")]
    pub r#file_system: Box<Option<super::super::types::appservice::WindowsWebAppSlotLogsHttpLogsFileSystem>>,
}
