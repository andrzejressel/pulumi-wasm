#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct PipelineDefinitionParameterValue {
    /// ID of the parameter value.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// Field value, expressed as a String.
    #[builder(into)]
    #[serde(rename = "stringValue")]
    pub r#string_value: Box<String>,
}
