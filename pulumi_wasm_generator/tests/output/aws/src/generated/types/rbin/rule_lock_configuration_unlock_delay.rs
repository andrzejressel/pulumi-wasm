#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct RuleLockConfigurationUnlockDelay {
    /// The unit of time in which to measure the unlock delay. Currently, the unlock delay can be measure only in days.
    #[builder(into)]
    #[serde(rename = "unlockDelayUnit")]
    pub r#unlock_delay_unit: Box<String>,
    /// The unlock delay period, measured in the unit specified for UnlockDelayUnit.
    #[builder(into)]
    #[serde(rename = "unlockDelayValue")]
    pub r#unlock_delay_value: Box<i32>,
}