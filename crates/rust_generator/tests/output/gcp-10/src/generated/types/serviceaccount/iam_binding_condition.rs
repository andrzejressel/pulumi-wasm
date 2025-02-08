#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IamBindingCondition {
    /// An optional description of the expression. This is a longer text which describes the expression, e.g. when hovered over it in a UI.
    /// 
    /// > **Warning:** This provider considers the `role` and condition contents (`title`+`description`+`expression`) as the
    /// identifier for the binding. This means that if any part of the condition is changed out-of-band, the provider will
    /// consider it to be an entirely different resource and will treat it as such.
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
