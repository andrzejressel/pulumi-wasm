#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceServiceConnectConfigurationLogConfigurationSecretOption {
    /// Name of the secret.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Secret to expose to the container. The supported values are either the full ARN of the AWS Secrets Manager secret or the full ARN of the parameter in the SSM Parameter Store.
    #[builder(into)]
    #[serde(rename = "valueFrom")]
    pub r#value_from: Box<String>,
}
