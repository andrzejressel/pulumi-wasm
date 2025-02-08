#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AwsClusterControlPlaneDatabaseEncryption {
    /// The ARN of the AWS KMS key used to encrypt cluster secrets.
    #[builder(into)]
    #[serde(rename = "kmsKeyArn")]
    pub r#kms_key_arn: Box<String>,
}
