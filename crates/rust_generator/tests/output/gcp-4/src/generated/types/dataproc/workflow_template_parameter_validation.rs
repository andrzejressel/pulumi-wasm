#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
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
