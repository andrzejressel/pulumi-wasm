#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplateParameterValidation {
    /// Validation based on regular expressions.
    #[builder(into, default)]
    #[serde(rename = "regex")]
    pub r#regex: Box<Option<super::super::types::dataproc::WorkflowTemplateParameterValidationRegex>>,
    /// Validation based on a list of allowed values.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<super::super::types::dataproc::WorkflowTemplateParameterValidationValues>>,
}
