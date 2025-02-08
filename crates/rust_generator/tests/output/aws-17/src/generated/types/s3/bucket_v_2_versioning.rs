#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketV2Versioning {
    /// Enable versioning. Once you version-enable a bucket, it can never return to an unversioned state. You can, however, suspend versioning on that bucket.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Enable MFA delete for either `Change the versioning state of your bucket` or `Permanently delete an object version`. Default is `false`. This cannot be used to toggle this setting but is available to allow managed buckets to reflect the state in AWS
    #[builder(into, default)]
    #[serde(rename = "mfaDelete")]
    pub r#mfa_delete: Box<Option<bool>>,
}
