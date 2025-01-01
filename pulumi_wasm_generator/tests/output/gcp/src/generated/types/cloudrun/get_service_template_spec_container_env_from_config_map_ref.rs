#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpecContainerEnvFromConfigMapRef {
    /// The ConfigMap to select from.
    #[builder(into)]
    #[serde(rename = "localObjectReferences")]
    pub r#local_object_references: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvFromConfigMapRefLocalObjectReference>>,
    /// Specify whether the ConfigMap must be defined
    #[builder(into)]
    #[serde(rename = "optional")]
    pub r#optional: Box<bool>,
}
