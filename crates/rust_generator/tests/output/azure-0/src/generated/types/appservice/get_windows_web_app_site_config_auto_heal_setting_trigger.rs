#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetWindowsWebAppSiteConfigAutoHealSettingTrigger {
    /// The amount of Private Memory used.
    #[builder(into)]
    #[serde(rename = "privateMemoryKb")]
    pub r#private_memory_kb: Box<i32>,
    /// A `requests` block as defined above.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerRequest>>,
    /// (Optional) One or more `slow_request_with_path` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequestWithPaths")]
    pub r#slow_request_with_paths: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerSlowRequestWithPath>>,
    /// A `slow_request` block as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequests")]
    pub r#slow_requests: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerSlowRequest>>,
    /// A `status_code` block as defined above.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Box<Vec<super::super::types::appservice::GetWindowsWebAppSiteConfigAutoHealSettingTriggerStatusCode>>,
}
