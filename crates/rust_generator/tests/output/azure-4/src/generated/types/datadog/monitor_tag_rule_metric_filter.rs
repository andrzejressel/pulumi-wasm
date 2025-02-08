#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct MonitorTagRuleMetricFilter {
    /// Allowed values Include or Exclude.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Name of the Tag.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Value of the Tag.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
