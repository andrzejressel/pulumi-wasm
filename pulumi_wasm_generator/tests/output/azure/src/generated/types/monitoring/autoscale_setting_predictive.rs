#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscaleSettingPredictive {
    /// Specifies the amount of time by which instances are launched in advance. It must be between `PT1M` and `PT1H` in ISO 8601 format.
    #[builder(into, default)]
    #[serde(rename = "lookAheadTime")]
    pub r#look_ahead_time: Box<Option<String>>,
    /// Specifies the predictive scale mode. Possible values are `Enabled` or `ForecastOnly`.
    #[builder(into)]
    #[serde(rename = "scaleMode")]
    pub r#scale_mode: Box<String>,
}