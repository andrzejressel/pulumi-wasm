#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DatabaseInstanceSettingsMaintenanceWindow {
    /// Day of week (`1-7`), starting on Monday
    #[builder(into, default)]
    #[serde(rename = "day")]
    pub r#day: Box<Option<i32>>,
    /// Hour of day (`0-23`), ignored if `day` not set
    #[builder(into, default)]
    #[serde(rename = "hour")]
    pub r#hour: Box<Option<i32>>,
    /// Receive updates after one week (`canary`) or after two weeks (`stable`) or after five weeks (`week5`) of notification.
    #[builder(into, default)]
    #[serde(rename = "updateTrack")]
    pub r#update_track: Box<Option<String>>,
}
