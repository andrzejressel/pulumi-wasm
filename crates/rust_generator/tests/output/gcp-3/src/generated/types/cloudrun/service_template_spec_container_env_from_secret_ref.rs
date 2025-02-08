#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ServiceTemplateSpecContainerEnvFromSecretRef {
    /// The Secret to select from.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "localObjectReference")]
    pub r#local_object_reference: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromSecretRefLocalObjectReference>>,
    /// Specify whether the Secret must be defined
    #[builder(into, default)]
    #[serde(rename = "optional")]
    pub r#optional: Box<Option<bool>>,
}
