#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketRetentionPolicy {
    /// If set to `true`, the bucket will be [locked](https://cloud.google.com/storage/docs/using-bucket-lock#lock-bucket) and permanently restrict edits to the bucket's retention policy.  Caution: Locking a bucket is an irreversible action.
    #[builder(into, default)]
    #[serde(rename = "isLocked")]
    pub r#is_locked: Box<Option<bool>>,
    /// The period of time, in seconds, that objects in the bucket must be retained and cannot be deleted, overwritten, or archived. The value must be less than 2,147,483,647 seconds.
    #[builder(into)]
    #[serde(rename = "retentionPeriod")]
    pub r#retention_period: Box<i32>,
}
