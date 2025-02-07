#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DataQualityJobDefinitionDataQualityJobOutputConfigMonitoringOutputsS3Output {
    /// The local path to the Amazon S3 storage location where Amazon SageMaker saves the results of a monitoring job. LocalPath is an absolute path for the output data. Defaults to `/opt/ml/processing/output`.
    #[builder(into, default)]
    #[serde(rename = "localPath")]
    pub r#local_path: Box<Option<String>>,
    /// Whether to upload the results of the monitoring job continuously or after the job completes. Valid values are `Continuous` or `EndOfJob`
    #[builder(into, default)]
    #[serde(rename = "s3UploadMode")]
    pub r#s_3_upload_mode: Box<Option<String>>,
    /// A URI that identifies the Amazon S3 storage location where Amazon SageMaker saves the results of a monitoring job.
    #[builder(into)]
    #[serde(rename = "s3Uri")]
    pub r#s_3_uri: Box<String>,
}
