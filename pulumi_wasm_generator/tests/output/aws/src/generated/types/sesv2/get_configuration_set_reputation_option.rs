#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationSetReputationOption {
    /// The date and time (in Unix time) when the reputation metrics were last given a fresh start.
    #[builder(into)]
    #[serde(rename = "lastFreshStart")]
    pub r#last_fresh_start: Box<String>,
    /// Specifies whether tracking of reputation metrics is enabled.
    #[builder(into)]
    #[serde(rename = "reputationMetricsEnabled")]
    pub r#reputation_metrics_enabled: Box<bool>,
}
