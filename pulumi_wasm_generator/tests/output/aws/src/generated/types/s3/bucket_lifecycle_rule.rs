#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketLifecycleRule {
    /// Specifies the number of days after initiating a multipart upload when the multipart upload must be completed.
    #[builder(into, default)]
    #[serde(rename = "abortIncompleteMultipartUploadDays")]
    pub r#abort_incomplete_multipart_upload_days: Box<Option<i32>>,
    /// Specifies lifecycle rule status.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Specifies a period in the object's expire (documented below).
    #[builder(into, default)]
    #[serde(rename = "expiration")]
    pub r#expiration: Box<Option<super::super::types::s3::BucketLifecycleRuleExpiration>>,
    /// Unique identifier for the rule. Must be less than or equal to 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies when noncurrent object versions expire (documented below).
    #[builder(into, default)]
    #[serde(rename = "noncurrentVersionExpiration")]
    pub r#noncurrent_version_expiration: Box<Option<super::super::types::s3::BucketLifecycleRuleNoncurrentVersionExpiration>>,
    /// Specifies when noncurrent object versions transitions (documented below).
    /// 
    /// At least one of `abort_incomplete_multipart_upload_days`, `expiration`, `transition`, `noncurrent_version_expiration`, `noncurrent_version_transition` must be specified.
    #[builder(into, default)]
    #[serde(rename = "noncurrentVersionTransitions")]
    pub r#noncurrent_version_transitions: Box<Option<Vec<super::super::types::s3::BucketLifecycleRuleNoncurrentVersionTransition>>>,
    /// Object key prefix identifying one or more objects to which the rule applies.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Specifies object tags key and value.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Specifies a period in the object's transitions (documented below).
    #[builder(into, default)]
    #[serde(rename = "transitions")]
    pub r#transitions: Box<Option<Vec<super::super::types::s3::BucketLifecycleRuleTransition>>>,
}
