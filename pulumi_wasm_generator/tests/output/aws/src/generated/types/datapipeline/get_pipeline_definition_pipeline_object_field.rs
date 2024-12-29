#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetPipelineDefinitionPipelineObjectField {
    /// Field identifier.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Field value, expressed as the identifier of another object
    #[builder(into)]
    #[serde(rename = "refValue")]
    pub r#ref_value: Box<String>,
    /// Field value, expressed as a String.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<String>,
}
