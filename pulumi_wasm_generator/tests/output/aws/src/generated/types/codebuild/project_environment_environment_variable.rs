#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ProjectEnvironmentEnvironmentVariable {
    /// Environment variable's name or key.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Type of environment variable. Valid values: `PARAMETER_STORE`, `PLAINTEXT`, `SECRETS_MANAGER`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// Environment variable's value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}