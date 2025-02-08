#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetDomainCognitoOption {
    /// Enabled disabled toggle for off-peak update window
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// Cognito Identity pool used by the domain.
    #[builder(into)]
    #[serde(rename = "identityPoolId")]
    pub r#identity_pool_id: Box<String>,
    /// IAM Role with the AmazonOpenSearchServiceCognitoAccess policy attached.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Cognito User pool used by the domain.
    #[builder(into)]
    #[serde(rename = "userPoolId")]
    pub r#user_pool_id: Box<String>,
}
