#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleConfigurationV2RuleNoncurrentVersionExpiration {
    /// Number of noncurrent versions Amazon S3 will retain. Must be a non-zero positive integer.
    #[builder(into, default)]
    #[serde(rename = "newerNoncurrentVersions")]
    pub r#newer_noncurrent_versions: Box<Option<String>>,
    /// Number of days an object is noncurrent before Amazon S3 can perform the associated action. Must be a positive integer.
    #[builder(into, default)]
    #[serde(rename = "noncurrentDays")]
    pub r#noncurrent_days: Box<Option<i32>>,
}
