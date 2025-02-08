#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketLifecycleConfigurationV2Rule {
    /// Configuration block that specifies the days since the initiation of an incomplete multipart upload that Amazon S3 will wait before permanently removing all parts of the upload. See below.
    #[builder(into, default)]
    #[serde(rename = "abortIncompleteMultipartUpload")]
    pub r#abort_incomplete_multipart_upload: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleAbortIncompleteMultipartUpload>>,
    /// Configuration block that specifies the expiration for the lifecycle of the object in the form of date, days and, whether the object has a delete marker. See below.
    #[builder(into, default)]
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleExpiration>>,
    /// Configuration block used to identify objects that a Lifecycle Rule applies to. See below. If not specified, the `rule` will default to using `prefix`.
    #[builder(into, default)]
    #[serde(rename = "filter")]
    pub r#filter: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleFilter>>,
    /// Unique identifier for the rule. The value cannot be longer than 255 characters.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Configuration block that specifies when noncurrent object versions expire. See below.
    #[builder(into, default)]
    #[serde(rename = "noncurrentVersionExpiration")]
    pub r#noncurrent_version_expiration: Box<Option<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionExpiration>>,
    /// Set of configuration blocks that specify the transition rule for the lifecycle rule that describes when noncurrent objects transition to a specific storage class. See below.
    #[builder(into, default)]
    #[serde(rename = "noncurrentVersionTransitions")]
    pub r#noncurrent_version_transitions: Box<Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition>>>,
    /// **DEPRECATED** Use `filter` instead. This has been deprecated by Amazon S3. Prefix identifying one or more objects to which the rule applies. Defaults to an empty string (`""`) if `filter` is not specified.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Whether the rule is currently being applied. Valid values: `Enabled` or `Disabled`.
    #[builder(into)]
    #[serde(rename = "status")]
    pub r#status: Box<String>,
    /// Set of configuration blocks that specify when an Amazon S3 object transitions to a specified storage class. See below.
    #[builder(into, default)]
    #[serde(rename = "transitions")]
    pub r#transitions: Box<Option<Vec<super::super::types::s3::BucketLifecycleConfigurationV2RuleTransition>>>,
}
