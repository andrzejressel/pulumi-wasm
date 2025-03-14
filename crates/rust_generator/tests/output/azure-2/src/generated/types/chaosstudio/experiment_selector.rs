#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ExperimentSelector {
    /// A list of Chaos Studio Target IDs that should be part of this Selector.
    #[builder(into)]
    #[serde(rename = "chaosStudioTargetIds")]
    pub r#chaos_studio_target_ids: Box<Vec<String>>,
    /// The name of this Selector.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
