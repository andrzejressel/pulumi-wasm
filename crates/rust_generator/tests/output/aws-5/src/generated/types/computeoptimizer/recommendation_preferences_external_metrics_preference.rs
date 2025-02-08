#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct RecommendationPreferencesExternalMetricsPreference {
    /// The source options for external metrics preferences. Valid values: `Datadog`, `Dynatrace`, `NewRelic`, `Instana`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
}
