#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBucketSoftDeletePolicy {
    /// Server-determined value that indicates the time from which the policy, or one with a greater retention, was effective. This value is in RFC 3339 format.
    #[builder(into)]
    #[serde(rename = "effectiveTime")]
    pub r#effective_time: Box<String>,
    /// The duration in seconds that soft-deleted objects in the bucket will be retained and cannot be permanently deleted. Default value is 604800.
    #[builder(into)]
    #[serde(rename = "retentionDurationSeconds")]
    pub r#retention_duration_seconds: Box<i32>,
}
