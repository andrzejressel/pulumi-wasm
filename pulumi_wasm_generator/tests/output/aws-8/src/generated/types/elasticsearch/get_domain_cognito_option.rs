#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDomainCognitoOption {
    /// Whether node to node encryption is enabled.
    #[builder(into)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<bool>,
    /// The Cognito Identity pool used by the domain.
    #[builder(into)]
    #[serde(rename = "identityPoolId")]
    pub r#identity_pool_id: Box<String>,
    /// The IAM Role with the AmazonESCognitoAccess policy attached.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The Cognito User pool used by the domain.
    #[builder(into)]
    #[serde(rename = "userPoolId")]
    pub r#user_pool_id: Box<String>,
}
