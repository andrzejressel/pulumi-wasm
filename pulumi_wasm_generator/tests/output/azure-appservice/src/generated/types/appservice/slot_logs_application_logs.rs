#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SlotLogsApplicationLogs {
    /// An `azure_blob_storage` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "azureBlobStorage")]
    pub r#azure_blob_storage: Box<Option<super::super::types::appservice::SlotLogsApplicationLogsAzureBlobStorage>>,
    /// The file system log level. Possible values are `Off`, `Error`, `Warning`, `Information`, and `Verbose`. Defaults to `Off`.
    #[builder(into, default)]
    #[serde(rename = "fileSystemLevel")]
    pub r#file_system_level: Box<Option<String>>,
}