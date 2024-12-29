#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BucketAclV2AccessControlPolicyGrant {
    /// Configuration block for the person being granted permissions. See below.
    #[builder(into, default)]
    #[serde(rename = "grantee")]
    pub r#grantee: Box<Option<super::super::types::s3::BucketAclV2AccessControlPolicyGrantGrantee>>,
    /// Logging permissions assigned to the grantee for the bucket. Valid values: `FULL_CONTROL`, `WRITE`, `WRITE_ACP`, `READ`, `READ_ACP`. See [What permissions can I grant?](https://docs.aws.amazon.com/AmazonS3/latest/userguide/acl-overview.html#permissions) for more details about what each permission means in the context of buckets.
    #[builder(into)]
    #[serde(rename = "permission")]
    pub r#permission: Box<String>,
}
