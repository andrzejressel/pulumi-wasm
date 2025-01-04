#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscaleSettingProfileRuleScaleAction {
    /// The amount of time to wait since the last scaling action before this action occurs. Must be between 1 minute and 1 week and formatted as a ISO 8601 string.
    #[builder(into)]
    #[serde(rename = "cooldown")]
    pub r#cooldown: Box<String>,
    /// The scale direction. Possible values are `Increase` and `Decrease`.
    #[builder(into)]
    #[serde(rename = "direction")]
    pub r#direction: Box<String>,
    /// The type of action that should occur. Possible values are `ChangeCount`, `ExactCount`, `PercentChangeCount` and `ServiceAllowedNextValue`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
    /// The number of instances involved in the scaling action.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<i32>,
}
