#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointConfigurationAsyncInferenceConfigOutputConfig {
    /// The Amazon Web Services Key Management Service (Amazon Web Services KMS) key that Amazon SageMaker uses to encrypt the asynchronous inference output in Amazon S3.
    #[builder(into, default)]
    #[serde(rename = "kmsKeyId")]
    pub r#kms_key_id: Box<Option<String>>,
    /// Specifies the configuration for notifications of inference results for asynchronous inference.
    #[builder(into, default)]
    #[serde(rename = "notificationConfig")]
    pub r#notification_config: Box<Option<super::super::types::sagemaker::EndpointConfigurationAsyncInferenceConfigOutputConfigNotificationConfig>>,
    /// The Amazon S3 location to upload failure inference responses to.
    #[builder(into, default)]
    #[serde(rename = "s3FailurePath")]
    pub r#s_3_failure_path: Box<Option<String>>,
    /// The Amazon S3 location to upload inference responses to.
    #[builder(into)]
    #[serde(rename = "s3OutputPath")]
    pub r#s_3_output_path: Box<String>,
}
