#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightsReportConfigObjectMetadataReportOptionsStorageDestinationOptions {
    /// The destination bucket that stores the generated inventory reports.
    #[builder(into)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<String>,
    /// The path within the destination bucket to store generated inventory reports.
    #[builder(into, default)]
    #[serde(rename = "destinationPath")]
    pub r#destination_path: Box<Option<String>>,
}
