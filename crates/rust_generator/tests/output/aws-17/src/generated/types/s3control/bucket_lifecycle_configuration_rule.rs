#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleConfigurationRule {
    /// Configuration block containing settings for abort incomplete multipart upload.
    #[builder(into, default)]
    #[serde(rename = "abortIncompleteMultipartUpload")]
    pub r#abort_incomplete_multipart_upload: Box<Option<super::super::types::s3control::BucketLifecycleConfigurationRuleAbortIncompleteMultipartUpload>>,
    /// Configuration block containing settings for expiration of objects.
    #[builder(into, default)]
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<super::super::types::s3control::BucketLifecycleConfigurationRuleExpiration>>,
    /// Configuration block containing settings for filtering.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::s3control::BucketLifecycleConfigurationRuleFilter>>,
    /// Unique identifier for the rule.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Status of the rule. Valid values: `Enabled` and `Disabled`. Defaults to `Enabled`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
}
