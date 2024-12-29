#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketV2LifecycleRule {
    /// Specifies the number of days after initiating a multipart upload when the multipart upload must be completed.
    #[builder(into, default)]
    #[serde(rename = "abortIncompleteMultipartUploadDays")]
    pub r#abort_incomplete_multipart_upload_days: Box<Option<i32>>,
    /// Specifies lifecycle rule status.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Specifies a period in the object's expire. See Expiration below for details.
    #[builder(into, default)]
    #[serde(rename = "expirations")]
    pub r#expirations: Box<Option<Vec<super::super::types::s3::BucketV2LifecycleRuleExpiration>>>,
    /// Unique identifier for the rule. Must be less than or equal to 255 characters in length.
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// Specifies when noncurrent object versions expire. See Noncurrent Version Expiration below for details.
    #[builder(into, default)]
    #[serde(rename = "noncurrentVersionExpirations")]
    pub r#noncurrent_version_expirations: Box<Option<Vec<super::super::types::s3::BucketV2LifecycleRuleNoncurrentVersionExpiration>>>,
    /// Specifies when noncurrent object versions transitions. See Noncurrent Version Transition below for details.
    #[builder(into, default)]
    #[serde(rename = "noncurrentVersionTransitions")]
    pub r#noncurrent_version_transitions: Box<Option<Vec<super::super::types::s3::BucketV2LifecycleRuleNoncurrentVersionTransition>>>,
    /// Object key prefix identifying one or more objects to which the rule applies.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// Specifies object tags key and value.
    #[builder(into, default)]
    #[serde(rename = "tags")]
    pub r#tags: Box<Option<std::collections::HashMap<String, String>>>,
    /// Specifies a period in the object's transitions. See Transition below for details.
    #[builder(into, default)]
    #[serde(rename = "transitions")]
    pub r#transitions: Box<Option<Vec<super::super::types::s3::BucketV2LifecycleRuleTransition>>>,
}
