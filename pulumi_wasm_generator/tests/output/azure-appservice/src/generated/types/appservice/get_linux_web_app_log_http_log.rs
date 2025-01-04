#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppLogHttpLog {
    /// A `azure_blob_storage` block as defined above.
    #[builder(into)]
    #[serde(rename = "azureBlobStorages")]
    pub r#azure_blob_storages: Box<Vec<super::super::types::appservice::GetLinuxWebAppLogHttpLogAzureBlobStorage>>,
    /// A `file_system` block as defined above.
    #[builder(into)]
    #[serde(rename = "fileSystems")]
    pub r#file_systems: Box<Vec<super::super::types::appservice::GetLinuxWebAppLogHttpLogFileSystem>>,
}
