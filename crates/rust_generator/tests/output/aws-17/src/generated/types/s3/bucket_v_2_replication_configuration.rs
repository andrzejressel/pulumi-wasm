#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct BucketV2ReplicationConfiguration {
    /// ARN of the IAM role for Amazon S3 to assume when replicating the objects.
    #[builder(into)]
    #[serde(rename = "role")]
    pub r#role: Box<String>,
    /// Specifies the rules managing the replication (documented below).
    #[builder(into)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Vec<super::super::types::s3::BucketV2ReplicationConfigurationRule>>,
}
