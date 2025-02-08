#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetServiceTemplateContainerEnvValueSourceSecretKeyRef {
    /// The name of the secret in Cloud Secret Manager. Format: {secretName} if the secret is in the same project. projects/{project}/secrets/{secretName} if the secret is in a different project.
    #[builder(into)]
    #[serde(rename = "secret")]
    pub r#secret: Box<String>,
    /// The Cloud Secret Manager secret version. Can be 'latest' for the latest value or an integer for a specific version.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
