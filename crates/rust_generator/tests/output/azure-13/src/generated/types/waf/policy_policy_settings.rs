#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyPolicySettings {
    /// Describes if the policy is in enabled state or disabled state. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    /// Whether the firewall should block a request with upload size greater then `file_upload_limit_in_mb`.
    #[builder(into, default)]
    #[serde(rename = "fileUploadEnforcement")]
    pub r#file_upload_enforcement: Box<Option<bool>>,
    /// The File Upload Limit in MB. Accepted values are in the range `1` to `4000`. Defaults to `100`.
    #[builder(into, default)]
    #[serde(rename = "fileUploadLimitInMb")]
    pub r#file_upload_limit_in_mb: Box<Option<i32>>,
    /// Specifies the JavaScript challenge cookie validity lifetime in minutes. The user is challenged after the lifetime expires. Accepted values are in the range `5` to `1440`. Defaults to `30`.
    #[builder(into, default)]
    #[serde(rename = "jsChallengeCookieExpirationInMinutes")]
    pub r#js_challenge_cookie_expiration_in_minutes: Box<Option<i32>>,
    /// One `log_scrubbing` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "logScrubbing")]
    pub r#log_scrubbing: Box<Option<super::super::types::waf::PolicyPolicySettingsLogScrubbing>>,
    /// The Maximum Request Body Size in KB. Accepted values are in the range `8` to `2000`. Defaults to `128`.
    #[builder(into, default)]
    #[serde(rename = "maxRequestBodySizeInKb")]
    pub r#max_request_body_size_in_kb: Box<Option<i32>>,
    /// Describes if it is in detection mode or prevention mode at the policy level. Valid values are `Detection` and `Prevention`. Defaults to `Prevention`.
    #[builder(into, default)]
    #[serde(rename = "mode")]
    pub r#mode: Box<Option<String>>,
    /// Is Request Body Inspection enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "requestBodyCheck")]
    pub r#request_body_check: Box<Option<bool>>,
    /// Whether the firewall should block a request with body size greater then `max_request_body_size_in_kb`. Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "requestBodyEnforcement")]
    pub r#request_body_enforcement: Box<Option<bool>>,
    /// Specifies the maximum request body inspection limit in KB for the Web Application Firewall. Defaults to `128`.
    #[builder(into, default)]
    #[serde(rename = "requestBodyInspectLimitInKb")]
    pub r#request_body_inspect_limit_in_kb: Box<Option<i32>>,
}
