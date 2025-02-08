#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TagRuleLogTagFilter {
    /// Valid actions for a filtering tag. Possible values are `Exclude` and `Include`. Exclusion takes priority over inclusion.
    #[builder(into)]
    #[serde(rename = "action")]
    pub r#action: Box<String>,
    /// Specifies the name (also known as the key) of the tag.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Specifies the value of the tag.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
