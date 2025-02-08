#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketLifecycleConfigurationV2RuleNoncurrentVersionTransition {
    /// Number of noncurrent versions Amazon S3 will retain. Must be a non-zero positive integer.
    #[builder(into, default)]
    #[serde(rename = "newerNoncurrentVersions")]
    pub r#newer_noncurrent_versions: Box<Option<String>>,
    /// Number of days an object is noncurrent before Amazon S3 can perform the associated action.
    #[builder(into, default)]
    #[serde(rename = "noncurrentDays")]
    pub r#noncurrent_days: Box<Option<i32>>,
    /// Class of storage used to store the object. Valid Values: `GLACIER`, `STANDARD_IA`, `ONEZONE_IA`, `INTELLIGENT_TIERING`, `DEEP_ARCHIVE`, `GLACIER_IR`.
    #[builder(into)]
    #[serde(rename = "storageClass")]
    pub r#storage_class: Box<String>,
}
