#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketObjectLockConfigurationRuleDefaultRetention {
    /// The number of days that you want to specify for the default retention period.
    #[builder(into, default)]
    #[serde(rename = "days")]
    pub r#days: Box<Option<i32>>,
    /// The default Object Lock retention mode you want to apply to new objects placed in this bucket. Valid values are `GOVERNANCE` and `COMPLIANCE`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// The number of years that you want to specify for the default retention period.
    /// 
    /// Either `days` or `years` must be specified, but not both.
    /// 
    /// > **NOTE on `object_lock_configuration`:** You can only enable S3 Object Lock for new buckets. If you need to turn on S3 Object Lock for an existing bucket, please contact AWS Support.
    /// When you create a bucket with S3 Object Lock enabled, Amazon S3 automatically enables versioning for the bucket.
    /// Once you create a bucket with S3 Object Lock enabled, you can't disable Object Lock or suspend versioning for the bucket.
    #[builder(into, default)]
    #[serde(rename = "years")]
    pub r#years: Box<Option<i32>>,
}
