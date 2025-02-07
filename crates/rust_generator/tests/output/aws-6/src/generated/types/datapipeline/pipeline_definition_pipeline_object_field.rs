#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PipelineDefinitionPipelineObjectField {
    /// Field identifier.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Field value, expressed as the identifier of another object
    #[builder(into, default)]
    #[serde(rename = "refValue")]
    pub r#ref_value: Box<Option<String>>,
    /// Field value, expressed as a String.
    #[builder(into, default)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<Option<String>>,
}
