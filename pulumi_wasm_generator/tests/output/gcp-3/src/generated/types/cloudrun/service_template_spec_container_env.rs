#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceTemplateSpecContainerEnv {
    /// Name of the environment variable.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Defaults to "".
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
    /// Source for the environment variable's value. Only supports secret_key_ref.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "valueFrom")]
    pub r#value_from: Box<Option<super::super::types::cloudrun::ServiceTemplateSpecContainerEnvValueFrom>>,
}
