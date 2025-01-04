#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxWebAppSlotSiteConfigAutoHealSettingTriggerRequests {
    /// The number of requests in the specified `interval` to trigger this rule.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The interval in `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<String>,
}
