#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateSpecContainerEnv {
    /// The name of the Cloud Run Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Defaults to "".
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
    /// Source for the environment variable's value. Only supports secret_key_ref.
    #[builder(into)]
    #[serde(rename = "valueFroms")]
    pub r#value_froms: Box<Vec<super::super::types::cloudrun::GetServiceTemplateSpecContainerEnvValueFrom>>,
}
