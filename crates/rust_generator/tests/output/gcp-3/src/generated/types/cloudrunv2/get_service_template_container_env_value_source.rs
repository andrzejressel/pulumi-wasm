#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateContainerEnvValueSource {
    /// Selects a secret and a specific version from Cloud Secret Manager.
    #[builder(into)]
    #[serde(rename = "secretKeyReves")]
    pub r#secret_key_reves: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateContainerEnvValueSourceSecretKeyRef>>,
}
