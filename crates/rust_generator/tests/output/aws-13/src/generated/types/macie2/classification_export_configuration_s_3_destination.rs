#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ClassificationExportConfigurationS3Destination {
    /// The Amazon S3 bucket name in which Amazon Macie exports the data classification results.
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    /// The object key for the bucket in which Amazon Macie exports the data classification results.
    #[builder(into, default)]
    #[serde(rename = "keyPrefix")]
    pub r#key_prefix: Box<Option<String>>,
    /// Amazon Resource Name (ARN) of the KMS key to be used to encrypt the data.
    /// 
    /// Additional information can be found in the [Storing and retaining sensitive data discovery results with Amazon Macie for AWS Macie documentation](https://docs.aws.amazon.com/macie/latest/user/discovery-results-repository-s3.html).
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<String>,
}
