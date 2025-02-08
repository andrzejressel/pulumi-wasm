#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ActionHttpRunAfter {
    /// Specifies the name of the precedent HTTP Action.
    #[builder(into)]
    #[serde(rename = "actionName")]
    pub r#action_name: Box<String>,
    /// Specifies the expected result of the precedent HTTP Action, only after which the current HTTP Action will be triggered. Possible values include `Succeeded`, `Failed`, `Skipped` and `TimedOut`.
    #[builder(into)]
    #[serde(rename = "actionResult")]
    pub r#action_result: Box<String>,
}
