#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagementProjectSecurityHealthAnalyticsCustomModuleCustomConfigPredicate {
    /// Description of the expression. This is a longer text which describes the
    /// expression, e.g. when hovered over it in a UI.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Textual representation of an expression in Common Expression Language syntax.
    #[builder(into)]
    #[serde(rename = "expression")]
    pub r#expression: Box<String>,
    /// String indicating the location of the expression for error reporting, e.g. a
    /// file name and a position in the file.
    #[builder(into, default)]
    #[serde(rename = "location")]
    pub r#location: Box<Option<String>>,
    /// Title for the expression, i.e. a short string describing its purpose. This can
    /// be used e.g. in UIs which allow to enter the expression.
    #[builder(into, default)]
    #[serde(rename = "title")]
    pub r#title: Box<Option<String>>,
}
