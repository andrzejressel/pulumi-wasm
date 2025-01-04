#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LinuxWebAppSlotSiteConfigAutoHealSettingTrigger {
    /// A `requests` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "requests")]
    pub r#requests: Box<Option<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerRequests>>,
    /// A `slow_request` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "slowRequest")]
    pub r#slow_request: Box<Option<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequest>>,
    /// One or more `slow_request_with_path` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "slowRequestWithPaths")]
    pub r#slow_request_with_paths: Box<Option<Vec<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerSlowRequestWithPath>>>,
    /// One or more `status_code` blocks as defined above.
    #[builder(into, default)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Box<Option<Vec<super::super::types::appservice::LinuxWebAppSlotSiteConfigAutoHealSettingTriggerStatusCode>>>,
}
