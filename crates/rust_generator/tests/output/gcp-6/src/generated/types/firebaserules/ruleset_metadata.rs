#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct RulesetMetadata {
    /// Services that this ruleset has declarations for (e.g., "cloud.firestore"). There may be 0+ of these.
    #[builder(into, default)]
    #[serde(rename = "services")]
    pub r#services: Box<Option<Vec<String>>>,
}
