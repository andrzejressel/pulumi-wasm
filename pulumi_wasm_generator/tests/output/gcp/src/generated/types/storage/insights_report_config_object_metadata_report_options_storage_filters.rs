#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InsightsReportConfigObjectMetadataReportOptionsStorageFilters {
    /// The filter to use when specifying which bucket to generate inventory reports for.
    #[builder(into, default)]
    #[serde(rename = "bucket")]
    pub r#bucket: Box<Option<String>>,
}
