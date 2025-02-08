#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DatabaseAclConfiguration {
    /// Amazon S3 canned ACL that Athena should specify when storing query results. Valid value is `BUCKET_OWNER_FULL_CONTROL`.
    /// 
    /// > **NOTE:** When Athena queries are executed, result files may be created in the specified bucket. Consider using `force_destroy` on the bucket too in order to avoid any problems when destroying the bucket.
    #[builder(into)]
    #[serde(rename = "s3AclOption")]
    pub r#s_3_acl_option: Box<String>,
}
