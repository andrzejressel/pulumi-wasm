#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputs {
    /// The Amazon S3 storage location where the results of a monitoring job are saved. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "s3Output")]
    pub r#s_3_output: Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputsS3Output>,
}
