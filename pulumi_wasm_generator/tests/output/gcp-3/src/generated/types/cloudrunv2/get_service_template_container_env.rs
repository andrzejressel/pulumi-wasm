#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetServiceTemplateContainerEnv {
    /// The name of the Cloud Run v2 Service.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Literal value of the environment variable. Defaults to "" and the maximum allowed length is 32768 characters. Variable references are not supported in Cloud Run.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
    /// Source for the environment variable's value.
    #[builder(into)]
    #[serde(rename = "valueSources")]
    pub r#value_sources: Box<Vec<super::super::types::cloudrunv2::GetServiceTemplateContainerEnvValueSource>>,
}
