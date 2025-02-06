#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityJobInput {
    /// Input object for the batch transform job. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "batchTransformInput")]
    pub r#batch_transform_input: Box<Option<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInputBatchTransformInput>>,
    /// Input object for the endpoint. Fields are documented below.
    #[builder(into, default)]
    #[serde(rename = "endpointInput")]
    pub r#endpoint_input: Box<Option<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobInputEndpointInput>>,
}
