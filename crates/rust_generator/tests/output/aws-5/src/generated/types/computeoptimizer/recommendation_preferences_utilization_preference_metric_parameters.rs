#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RecommendationPreferencesUtilizationPreferenceMetricParameters {
    /// The headroom value in percentage used for the specified metric parameter. Valid values: `PERCENT_30`, `PERCENT_20`, `PERCENT_10`, `PERCENT_0`.
    #[builder(into)]
    #[serde(rename = "headroom")]
    pub r#headroom: Box<String>,
    /// The threshold value used for the specified metric parameter. You can only specify the threshold value for CPU utilization. Valid values: `P90`, `P95`, `P99_5`.
    #[builder(into, default)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<Option<String>>,
}
