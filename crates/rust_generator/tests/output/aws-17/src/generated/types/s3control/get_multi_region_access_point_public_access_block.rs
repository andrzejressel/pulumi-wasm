#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetMultiRegionAccessPointPublicAccessBlock {
    /// Specifies whether Amazon S3 should block public access control lists (ACLs). When set to `true` causes the following behavior:
    /// * PUT Bucket acl and PUT Object acl calls fail if the specified ACL is public.
    /// * PUT Object calls fail if the request includes a public ACL.
    /// * PUT Bucket calls fail if the request includes a public ACL.
    #[builder(into)]
    #[serde(rename = "blockPublicAcls")]
    pub r#block_public_acls: Box<bool>,
    /// Specifies whether Amazon S3 should block public bucket policies for buckets in this account. When set to `true` causes Amazon S3 to:
    /// * Reject calls to PUT Bucket policy if the specified bucket policy allows public access.
    #[builder(into)]
    #[serde(rename = "blockPublicPolicy")]
    pub r#block_public_policy: Box<bool>,
    /// Specifies whether Amazon S3 should ignore public ACLs for buckets in this account. When set to `true` causes Amazon S3 to:
    /// * Ignore all public ACLs on buckets in this account and any objects that they contain.
    #[builder(into)]
    #[serde(rename = "ignorePublicAcls")]
    pub r#ignore_public_acls: Box<bool>,
    /// Specifies whether Amazon S3 should restrict public bucket policies for buckets in this account. When set to `true`:
    /// * Only the bucket owner and AWS Services can access buckets with public policies.
    #[builder(into)]
    #[serde(rename = "restrictPublicBuckets")]
    pub r#restrict_public_buckets: Box<bool>,
}
