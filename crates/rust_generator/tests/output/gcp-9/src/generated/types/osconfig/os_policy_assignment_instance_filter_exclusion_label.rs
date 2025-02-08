#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct OsPolicyAssignmentInstanceFilterExclusionLabel {
    /// Labels are identified by key/value pairs in this map.
    /// A VM should contain all the key/value pairs specified in this map to be
    /// selected.
    #[builder(into, default)]
    #[serde(rename = "labels")]
    pub r#labels: Box<Option<std::collections::HashMap<String, String>>>,
}
