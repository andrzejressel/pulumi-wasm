#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetUserPoolClientAnalyticsConfiguration {
    /// (Optional) Application ARN for an Amazon Pinpoint application. Conflicts with `external_id` and `role_arn`.
    #[builder(into)]
    #[serde(rename = "applicationArn")]
    pub r#application_arn: Box<String>,
    /// (Optional) Application ID for an Amazon Pinpoint application.
    #[builder(into)]
    #[serde(rename = "applicationId")]
    pub r#application_id: Box<String>,
    /// (Optional) ID for the Analytics Configuration. Conflicts with `application_arn`.
    #[builder(into)]
    #[serde(rename = "externalId")]
    pub r#external_id: Box<String>,
    /// (Optional) ARN of an IAM role that authorizes Amazon Cognito to publish events to Amazon Pinpoint analytics. Conflicts with `application_arn`.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// (Optional) If set to `true`, Amazon Cognito will include user data in the events it publishes to Amazon Pinpoint analytics.
    #[builder(into)]
    #[serde(rename = "userDataShared")]
    pub r#user_data_shared: Box<bool>,
}
