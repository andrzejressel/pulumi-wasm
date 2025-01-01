#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VolumeTieringPolicy {
    /// Optional. Time in days to mark the volume's data block as cold and make it eligible for tiering, can be range from 7-183.
    /// Default is 31.
    #[builder(into, default)]
    #[serde(rename = "coolingThresholdDays")]
    pub r#cooling_threshold_days: Box<Option<i32>>,
    /// Optional. Flag indicating if the volume has tiering policy enable/pause. Default is PAUSED.
    /// Default value is `PAUSED`.
    /// Possible values are: `ENABLED`, `PAUSED`.
    #[builder(into, default)]
    #[serde(rename = "tierAction")]
    pub r#tier_action: Box<Option<String>>,
}
