#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSlotSiteConfigAutoHealSettingTrigger {
    /// The amount of Private Memory to be consumed for this rule to trigger. Possible values are between `102400` and `13631488`.
    #[builder(into, default)]
    #[serde(rename = "privateMemoryKb")]
    pub r#private_memory_kb: Box<Option<i32>>,
    /// A `requests` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "requests")]
    pub r#requests: Box<Option<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerRequests>>,
    /// A `slow_request` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "slowRequest")]
    pub r#slow_request: Box<Option<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequest>>,
    /// One or more `slow_request_with_path` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "slowRequestWithPaths")]
    pub r#slow_request_with_paths: Box<Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequestWithPath>>>,
    /// One or more `status_code` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Box<Option<Vec<super::super::types::appservice::WindowsWebAppSlotSiteConfigAutoHealSettingTriggerStatusCode>>>,
}
