#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DeploymentGroupOnPremisesInstanceTagFilter {
    /// The key of the tag filter.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// The type of the tag filter, either `KEY_ONLY`, `VALUE_ONLY`, or `KEY_AND_VALUE`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// The value of the tag filter.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
