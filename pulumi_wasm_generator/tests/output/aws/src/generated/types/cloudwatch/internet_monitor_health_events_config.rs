#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InternetMonitorHealthEventsConfig {
    /// The health event threshold percentage set for availability scores.
    #[builder(into, default)]
    #[serde(rename = "availabilityScoreThreshold")]
    pub r#availability_score_threshold: Box<Option<f64>>,
    /// The health event threshold percentage set for performance scores.
    #[builder(into, default)]
    #[serde(rename = "performanceScoreThreshold")]
    pub r#performance_score_threshold: Box<Option<f64>>,
}