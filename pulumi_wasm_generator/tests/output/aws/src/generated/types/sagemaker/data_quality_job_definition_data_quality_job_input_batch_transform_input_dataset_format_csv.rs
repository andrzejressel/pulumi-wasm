#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormatCsv {
    /// Indicates if the CSV data has a header.
    #[builder(into, default)]
    #[serde(rename = "header")]
    pub r#header: Box<Option<bool>>,
}