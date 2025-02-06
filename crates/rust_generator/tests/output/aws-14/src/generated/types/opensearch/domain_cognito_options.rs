#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainCognitoOptions {
    /// Whether Amazon Cognito authentication with Dashboard is enabled or not. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// ID of the Cognito Identity Pool to use.
    #[builder(into)]
    #[serde(rename = "identityPoolId")]
    pub r#identity_pool_id: Box<String>,
    /// ARN of the IAM role that has the AmazonOpenSearchServiceCognitoAccess policy attached.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// ID of the Cognito User Pool to use.
    #[builder(into)]
    #[serde(rename = "userPoolId")]
    pub r#user_pool_id: Box<String>,
}
