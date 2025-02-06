#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplateParameter {
    /// Brief description of the parameter. Must not exceed 1024 characters.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Required. Paths to all fields that the parameter replaces. A field is allowed to appear in at most one parameter's list of field paths. A field path is similar in syntax to a .sparkJob.args
    #[builder(into)]
    #[serde(rename = "fields")]
    pub r#fields: Box<Vec<String>>,
    /// Required. Parameter name. The parameter name is used as the key, and paired with the parameter value, which are passed to the template when the template is instantiated. The name must contain only capital letters (A-Z), numbers (0-9), and underscores (_), and must not start with a number. The maximum length is 40 characters.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Validation rules to be applied to this parameter's value.
    #[builder(into, default)]
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<super::super::types::dataproc::WorkflowTemplateParameterValidation>>,
}
