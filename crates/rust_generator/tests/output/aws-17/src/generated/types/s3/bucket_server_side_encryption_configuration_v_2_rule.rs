#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketServerSideEncryptionConfigurationV2Rule {
    /// Single object for setting server-side encryption by default. See below.
    #[builder(into, default)]
    #[serde(rename = "applyServerSideEncryptionByDefault")]
    pub r#apply_server_side_encryption_by_default: Box<Option<super::super::types::s3::BucketServerSideEncryptionConfigurationV2RuleApplyServerSideEncryptionByDefault>>,
    /// Whether or not to use [Amazon S3 Bucket Keys](https://docs.aws.amazon.com/AmazonS3/latest/dev/bucket-key.html) for SSE-KMS.
    #[builder(into, default)]
    #[serde(rename = "bucketKeyEnabled")]
    pub r#bucket_key_enabled: Box<Option<bool>>,
}
