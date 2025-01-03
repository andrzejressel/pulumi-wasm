#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceSourceConfigurationAuthenticationConfiguration {
    /// ARN of the IAM role that grants the App Runner service access to a source repository. Required for ECR image repositories (but not for ECR Public)
    #[builder(into, default)]
    #[serde(rename = "accessRoleArn")]
    pub r#access_role_arn: Box<Option<String>>,
    /// ARN of the App Runner connection that enables the App Runner service to connect to a source repository. Required for GitHub code repositories.
    #[builder(into, default)]
    #[serde(rename = "connectionArn")]
    pub r#connection_arn: Box<Option<String>>,
}
