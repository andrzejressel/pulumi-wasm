#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecommendationPreferencesExternalMetricsPreference {
    /// The source options for external metrics preferences. Valid values: `Datadog`, `Dynatrace`, `NewRelic`, `Instana`.
    #[builder(into)]
    #[serde(rename = "source")]
    pub r#source: Box<String>,
}
