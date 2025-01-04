#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSlotLogsHttpLogsAzureBlobStorage {
    /// The time in days after which to remove blobs. A value of `0` means no retention.
    #[builder(into, default)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: Box<Option<i32>>,
    /// SAS url to an Azure blob container with read/write/list/delete permissions.
    #[builder(into)]
    #[serde(rename = "sasUrl")]
    pub r#sas_url: Box<String>,
}
