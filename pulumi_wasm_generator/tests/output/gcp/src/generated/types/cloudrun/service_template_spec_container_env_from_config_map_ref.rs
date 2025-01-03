#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateSpecContainerEnvFromConfigMapRef {
    /// The ConfigMap to select from.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "localObjectReference")]
    pub r#local_object_reference: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvFromConfigMapRefLocalObjectReference>>,
    /// Specify whether the ConfigMap must be defined
    #[builder(into, default)]
    #[serde(rename = "optional")]
    pub r#optional: Box<Option<bool>>,
}
