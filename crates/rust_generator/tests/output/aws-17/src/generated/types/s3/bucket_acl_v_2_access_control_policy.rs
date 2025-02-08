#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketAclV2AccessControlPolicy {
    /// Set of `grant` configuration blocks. See below.
    #[builder(into, default)]
    #[serde(rename = "grants")]
    pub r#grants: Box<Option<Vec<super::super::types::s3::BucketAclV2AccessControlPolicyGrant>>>,
    /// Configuration block for the bucket owner's display name and ID. See below.
    #[builder(into)]
    #[serde(rename = "owner")]
    pub r#owner: Box<super::super::types::s3::BucketAclV2AccessControlPolicyOwner>,
}
