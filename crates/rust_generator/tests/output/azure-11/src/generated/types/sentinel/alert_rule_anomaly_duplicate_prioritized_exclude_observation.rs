#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AlertRuleAnomalyDuplicatePrioritizedExcludeObservation {
    /// The description of the prioritized exclude observation.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// The excluded value per `description`.
    #[builder(into, default)]
    #[serde(rename = "exclude")]
    pub r#exclude: Box<Option<String>>,
    /// The name of the prioritized exclude observation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The prioritized value per `description`.
    #[builder(into, default)]
    #[serde(rename = "prioritize")]
    pub r#prioritize: Box<Option<String>>,
}
