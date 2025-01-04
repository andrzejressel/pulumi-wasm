#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SystemTopicEventSubscriptionSubjectFilter {
    /// Specifies if `subject_begins_with` and `subject_ends_with` case sensitive. This value
    #[builder(into, default)]
    #[serde(rename = "caseSensitive")]
    pub r#case_sensitive: Box<Option<bool>>,
    /// A string to filter events for an event subscription based on a resource path prefix.
    #[builder(into, default)]
    #[serde(rename = "subjectBeginsWith")]
    pub r#subject_begins_with: Box<Option<String>>,
    /// A string to filter events for an event subscription based on a resource path suffix.
    #[builder(into, default)]
    #[serde(rename = "subjectEndsWith")]
    pub r#subject_ends_with: Box<Option<String>>,
}
