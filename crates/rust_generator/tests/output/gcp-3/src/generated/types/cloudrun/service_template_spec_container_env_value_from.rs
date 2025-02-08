#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecContainerEnvValueFrom {
    /// Selects a key (version) of a secret in Secret Manager.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "secretKeyRef")]
    pub r#secret_key_ref: Box<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvValueFromSecretKeyRef>,
}
