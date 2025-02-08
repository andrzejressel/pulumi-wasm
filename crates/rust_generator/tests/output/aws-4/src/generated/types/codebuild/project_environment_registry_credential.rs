#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ProjectEnvironmentRegistryCredential {
    /// ARN or name of credentials created using AWS Secrets Manager.
    #[builder(into)]
    #[serde(rename = "credential")]
    pub r#credential: Box<String>,
    /// Service that created the credentials to access a private Docker registry. Valid value: `SECRETS_MANAGER` (AWS Secrets Manager).
    #[builder(into)]
    #[serde(rename = "credentialProvider")]
    pub r#credential_provider: Box<String>,
}
