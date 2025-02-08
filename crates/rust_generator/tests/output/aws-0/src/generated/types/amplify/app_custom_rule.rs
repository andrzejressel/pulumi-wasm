#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AppCustomRule {
    /// Condition for a URL rewrite or redirect rule, such as a country code.
    #[builder(into, default)]
    #[serde(rename = "condition")]
    pub r#condition: Box<Option<String>>,
    /// Source pattern for a URL rewrite or redirect rule.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
    /// Status code for a URL rewrite or redirect rule. Valid values: `200`, `301`, `302`, `404`, `404-200`.
    #[builder(into, default)]
    #[serde(rename = "status")]
    pub r#status: Box<Option<String>>,
    /// Target pattern for a URL rewrite or redirect rule.
    #[builder(into)]
    #[serde(rename = "target")]
    pub r#target: Box<String>,
}
