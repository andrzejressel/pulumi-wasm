#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketLifecycleConfigurationV2RuleAbortIncompleteMultipartUpload {
    /// Number of days after which Amazon S3 aborts an incomplete multipart upload.
    #[builder(into, default)]
    #[serde(rename = "daysAfterInitiation")]
    pub r#days_after_initiation: Box<Option<i32>>,
}
