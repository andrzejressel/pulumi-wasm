#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceIamMemberCondition {
    /// An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    /// 
    /// For `gcp.bigtable.InstanceIamPolicy` only:
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Textual representation of an expression in Common Expression Language syntax.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// A title for the expression, i.e. a short string describing its purpose.
    #[builder(into)]
    #[serde(rename = "title")]
    pub r#title: Box<String>,
}
