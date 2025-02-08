#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketSoftDeletePolicy {
    /// Server-determined value that indicates the time from which the policy, or one with a greater retention, was effective. This value is in RFC 3339 format.
    #[builder(into, default)]
    #[serde(rename = "effectiveTime")]
    pub r#effective_time: Box<Option<String>>,
    /// The duration in seconds that soft-deleted objects in the bucket will be retained and cannot be permanently deleted. Default value is 604800. The value must be in between 604800(7 days) and 7776000(90 days). **Note**: To disable the soft delete policy on a bucket, This field must be set to 0.
    #[builder(into, default)]
    #[serde(rename = "retentionDurationSeconds")]
    pub r#retention_duration_seconds: Box<Option<i32>>,
}
