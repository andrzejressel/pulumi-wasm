#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DataQualityJobDefinitionDataQualityJobOutputConfig {
    /// The AWS Key Management Service (AWS KMS) key that Amazon SageMaker uses to encrypt the model artifacts at rest using Amazon S3 server-side encryption.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Monitoring outputs for monitoring jobs. This is where the output of the periodic monitoring jobs is uploaded. Fields are documented below.
    #[builder(into)]
    #[serde(rename = "monitoringOutputs")]
    pub r#monitoring_outputs: Box<super::super::types::sagemaker::DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputs>,
}
