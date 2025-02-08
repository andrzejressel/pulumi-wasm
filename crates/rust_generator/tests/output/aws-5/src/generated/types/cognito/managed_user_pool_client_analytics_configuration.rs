#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedUserPoolClientAnalyticsConfiguration {
    /// Application ARN for an Amazon Pinpoint application. It conflicts with `external_id` and `role_arn`.
    #[builder(into, default)]
    #[serde(rename = "applicationArn")]
    pub r#application_arn: Box<Option<String>>,
    /// Unique identifier for an Amazon Pinpoint application.
    #[builder(into, default)]
    #[serde(rename = "applicationId")]
    pub r#application_id: Box<Option<String>>,
    /// ID for the Analytics Configuration and conflicts with `application_arn`.
    #[builder(into, default)]
    #[serde(rename = "externalId")]
    pub r#external_id: Box<Option<String>>,
    /// ARN of an IAM role that authorizes Amazon Cognito to publish events to Amazon Pinpoint analytics. It conflicts with `application_arn`.
    #[builder(into, default)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<Option<String>>,
    /// If `user_data_shared` is set to `true`, Amazon Cognito will include user data in the events it publishes to Amazon Pinpoint analytics.
    #[builder(into, default)]
    #[serde(rename = "userDataShared")]
    pub r#user_data_shared: Box<Option<bool>>,
}
