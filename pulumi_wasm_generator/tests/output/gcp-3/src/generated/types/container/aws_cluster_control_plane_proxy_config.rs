#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterControlPlaneProxyConfig {
    /// The ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration.
    #[builder(into)]
    #[serde(rename = "secretArn")]
    pub r#secret_arn: Box<String>,
    /// The version string of the AWS Secret Manager secret that contains the HTTP(S) proxy configuration.
    #[builder(into)]
    #[serde(rename = "secretVersion")]
    pub r#secret_version: Box<String>,
}
