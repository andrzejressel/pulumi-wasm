#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPipelineDefinitionParameterValue {
    /// ID of the object.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Field value, expressed as a String.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<String>,
}