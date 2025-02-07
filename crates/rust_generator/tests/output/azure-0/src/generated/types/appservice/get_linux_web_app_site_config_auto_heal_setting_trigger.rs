#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppSiteConfigAutoHealSettingTrigger {
    /// A `requests` block as defined above.
    #[builder(into)]
    #[serde(rename = "requests")]
    pub r#requests: Box<Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigAutoHealSettingTriggerRequest>>,
    /// (Optional) One or more `slow_request_with_path` blocks as defined above.
    #[builder(into)]
    #[serde(rename = "slowRequestWithPaths")]
    pub r#slow_request_with_paths: Box<Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigAutoHealSettingTriggerSlowRequestWithPath>>,
    /// A `slow_request` block as defined above.
    #[builder(into, default)]
    #[serde(rename = "slowRequests")]
    pub r#slow_requests: Box<Option<Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigAutoHealSettingTriggerSlowRequest>>>,
    /// A `status_code` block as defined above.
    #[builder(into)]
    #[serde(rename = "statusCodes")]
    pub r#status_codes: Box<Vec<super::super::types::appservice::GetLinuxWebAppSiteConfigAutoHealSettingTriggerStatusCode>>,
}
