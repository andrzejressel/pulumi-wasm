#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormat {
    /// The CSV dataset used in the monitoring job. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "csv")]
    pub r#csv: Box<Option<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormatCsv>>,
    /// The JSON dataset used in the monitoring job. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "json")]
    pub r#json: Box<Option<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInputBatchTransformInputDatasetFormatJson>>,
}
