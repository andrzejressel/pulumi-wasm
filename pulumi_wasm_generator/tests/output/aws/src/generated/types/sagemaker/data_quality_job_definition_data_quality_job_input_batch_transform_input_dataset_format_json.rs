#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormatJson {
    /// Indicates if the file should be read as a json object per line.
    #[builder(into, default)]
    #[serde(rename = "line")]
    pub r#line: Box<Option<bool>>,
}