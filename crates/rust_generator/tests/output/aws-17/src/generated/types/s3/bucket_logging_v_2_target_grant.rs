#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct BucketLoggingV2TargetGrant {
    /// Configuration block for the person being granted permissions. See below.
    #[builder(into)]
    #[serde(rename = "grantee")]
    pub r#grantee: Box<super::super::types::s3::BucketLoggingV2TargetGrantGrantee>,
    /// Logging permissions assigned to the grantee for the bucket. Valid values: `FULL_CONTROL`, `READ`, `WRITE`.
    #[builder(into)]
    #[serde(rename = "permission")]
    pub r#permission: Box<String>,
}
