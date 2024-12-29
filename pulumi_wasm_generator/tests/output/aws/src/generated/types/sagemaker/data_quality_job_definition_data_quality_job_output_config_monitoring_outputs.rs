#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputs {
    /// The Amazon S3 storage location where the results of a monitoring job are saved. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "s3Output")]
    pub r#s_3_output: Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputsS3Output>,
}
