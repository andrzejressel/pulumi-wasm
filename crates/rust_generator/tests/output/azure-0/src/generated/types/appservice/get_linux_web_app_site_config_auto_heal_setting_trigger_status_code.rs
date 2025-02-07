#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppSiteConfigAutoHealSettingTriggerStatusCode {
    /// The number of occurrences of the defined `status_code` in the specified `interval` on which to trigger this rule.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The time interval in the form `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<String>,
    /// The path to which this rule status code applies.
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
    /// The status code or range for this rule.
    #[builder(into)]
    #[serde(rename = "statusCodeRange")]
    pub r#status_code_range: Box<String>,
    /// The Request Sub Status of the Status Code.
    #[builder(into)]
    #[serde(rename = "subStatus")]
    pub r#sub_status: Box<i32>,
    /// The Win32 Status Code of the Request.
    #[builder(into)]
    #[serde(rename = "win32StatusCode")]
    pub r#win_32_status_code: Box<i32>,
}
