#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriberNotificationConfigurationHttpsNotificationConfiguration {
    /// The API key name for the notification subscription.
    #[builder(into, default)]
    #[serde(rename = "authorizationApiKeyName")]
    pub r#authorization_api_key_name: Box<Option<String>>,
    /// The API key value for the notification subscription.
    #[builder(into, default)]
    #[serde(rename = "authorizationApiKeyValue")]
    pub r#authorization_api_key_value: Box<Option<String>>,
    /// The subscription endpoint in Security Lake.
    /// If you prefer notification with an HTTPS endpoint, populate this field.
    #[builder(into)]
    #[serde(rename = "endpoint")]
    pub r#endpoint: Box<String>,
    /// The HTTP method used for the notification subscription.
    /// Valid values are `POST` and `PUT`.
    #[builder(into, default)]
    #[serde(rename = "httpMethod")]
    pub r#http_method: Box<Option<String>>,
    /// The Amazon Resource Name (ARN) of the EventBridge API destinations IAM role that you created.
    /// For more information about ARNs and how to use them in policies, see Managing data access and AWS Managed Policies in the Amazon Security Lake User Guide.
    #[builder(into)]
    #[serde(rename = "targetRoleArn")]
    pub r#target_role_arn: Box<String>,
}
