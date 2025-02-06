#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkgroupConfigurationResultConfiguration {
    /// That an Amazon S3 canned ACL should be set to control ownership of stored query results. See ACL Configuration below.
    #[builder(into, default)]
    #[serde(rename = "aclConfiguration")]
    pub r#acl_configuration: Box<Option<super::super::types::athena::WorkgroupConfigurationResultConfigurationAclConfiguration>>,
    /// Configuration block with encryption settings. See Encryption Configuration below.
    #[builder(into, default)]
    #[serde(rename = "encryptionConfiguration")]
    pub r#encryption_configuration: Box<Option<super::super::types::athena::WorkgroupConfigurationResultConfigurationEncryptionConfiguration>>,
    /// AWS account ID that you expect to be the owner of the Amazon S3 bucket.
    #[builder(into, default)]
    #[serde(rename = "expectedBucketOwner")]
    pub r#expected_bucket_owner: Box<Option<String>>,
    /// Location in Amazon S3 where your query results are stored, such as `s3://path/to/query/bucket/`. For more information, see [Queries and Query Result Files](https://docs.aws.amazon.com/athena/latest/ug/querying.html).
    #[builder(into, default)]
    #[serde(rename = "outputLocation")]
    pub r#output_location: Box<Option<String>>,
}
