#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AppServiceLogsHttpLogsAzureBlobStorage {
    /// The number of days to retain logs for.
    #[builder(into)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: Box<i32>,
    /// The URL to the storage container with a shared access signature token appended.
    #[builder(into)]
    #[serde(rename = "sasUrl")]
    pub r#sas_url: Box<String>,
}
