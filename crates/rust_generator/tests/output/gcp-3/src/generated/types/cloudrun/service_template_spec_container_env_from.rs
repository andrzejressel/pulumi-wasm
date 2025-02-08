#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ServiceTemplateSpecContainerEnvFrom {
    /// The ConfigMap to select from.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "configMapRef")]
    pub r#config_map_ref: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromConfigMapRef>>,
    /// An optional identifier to prepend to each key in the ConfigMap.
    #[builder(into, default)]
    #[serde(rename = "prefix")]
    pub r#prefix: Box<Option<String>>,
    /// The Secret to select from.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "secretRef")]
    pub r#secret_ref: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromSecretRef>>,
}
