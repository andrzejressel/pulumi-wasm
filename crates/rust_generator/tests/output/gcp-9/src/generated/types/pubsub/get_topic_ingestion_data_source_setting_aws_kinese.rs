#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTopicIngestionDataSourceSettingAwsKinese {
    /// AWS role ARN to be used for Federated Identity authentication with
    /// Kinesis. Check the Pub/Sub docs for how to set up this role and the
    /// required permissions that need to be attached to it.
    #[builder(into)]
    #[serde(rename = "awsRoleArn")]
    pub r#aws_role_arn: Box<String>,
    /// The Kinesis consumer ARN to used for ingestion in
    /// Enhanced Fan-Out mode. The consumer must be already
    /// created and ready to be used.
    #[builder(into)]
    #[serde(rename = "consumerArn")]
    pub r#consumer_arn: Box<String>,
    /// The GCP service account to be used for Federated Identity authentication
    /// with Kinesis (via a 'AssumeRoleWithWebIdentity' call for the provided
    /// role). The 'awsRoleArn' must be set up with 'accounts.google.com:sub'
    /// equals to this service account number.
    #[builder(into)]
    #[serde(rename = "gcpServiceAccount")]
    pub r#gcp_service_account: Box<String>,
    /// The Kinesis stream ARN to ingest data from.
    #[builder(into)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: Box<String>,
}
