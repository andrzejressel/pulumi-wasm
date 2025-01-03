#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkgroupConfigurationResultConfigurationAclConfiguration {
    /// Amazon S3 canned ACL that Athena should specify when storing query results. Valid value is `BUCKET_OWNER_FULL_CONTROL`.
    #[builder(into)]
    #[serde(rename = "s3AclOption")]
    pub r#s_3_acl_option: Box<String>,
}
