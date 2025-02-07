#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsClusterControlPlaneAwsServicesAuthentication {
    /// The Amazon Resource Name (ARN) of the role that the Anthos Multi-Cloud API will assume when managing AWS resources on your account.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// Optional. An identifier for the assumed role session. When unspecified, it defaults to `multicloud-service-agent`.
    #[builder(into, default)]
    #[serde(rename = "roleSessionName")]
    pub r#role_session_name: Box<Option<String>>,
}
