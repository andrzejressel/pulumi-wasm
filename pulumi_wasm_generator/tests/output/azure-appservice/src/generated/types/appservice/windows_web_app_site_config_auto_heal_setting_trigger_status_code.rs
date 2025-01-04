#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsWebAppSiteConfigAutoHealSettingTriggerStatusCode {
    /// The number of occurrences of the defined `status_code` in the specified `interval` on which to trigger this rule.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// The time interval in the form `hh:mm:ss`.
    #[builder(into)]
    #[serde(rename = "interval")]
    pub r#interval: Box<String>,
    /// The path to which this rule status code applies.
    #[builder(into, default)]
    #[serde(rename = "path")]
    pub r#path: Box<Option<String>>,
    /// The status code for this rule, accepts single status codes and status code ranges. e.g. `500` or `400-499`. Possible values are integers between `101` and `599`
    #[builder(into)]
    #[serde(rename = "statusCodeRange")]
    pub r#status_code_range: Box<String>,
    /// The Request Sub Status of the Status Code.
    #[builder(into, default)]
    #[serde(rename = "subStatus")]
    pub r#sub_status: Box<Option<i32>>,
    /// The Win32 Status Code of the Request.
    #[builder(into, default)]
    #[serde(rename = "win32StatusCode")]
    pub r#win_32_status_code: Box<Option<i32>>,
}
