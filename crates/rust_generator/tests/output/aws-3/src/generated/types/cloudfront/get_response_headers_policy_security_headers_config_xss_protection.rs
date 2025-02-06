#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetResponseHeadersPolicySecurityHeadersConfigXssProtection {
    /// Whether CloudFront includes the mode=block directive in the X-XSS-Protection header.
    #[builder(into)]
    #[serde(rename = "modeBlock")]
    pub r#mode_block: Box<bool>,
    /// Whether CloudFront overrides the X-XSS-Protection HTTP response header received from the origin with the one specified in this response headers policy.
    #[builder(into)]
    #[serde(rename = "override")]
    pub r#override_: Box<bool>,
    /// Boolean value that determines the value of the X-XSS-Protection HTTP response header. When this setting is true, the value of the X-XSS-Protection header is 1. When this setting is false, the value of the X-XSS-Protection header is 0.
    #[builder(into)]
    #[serde(rename = "protection")]
    pub r#protection: Box<bool>,
    /// Whether CloudFront sets a reporting URI in the X-XSS-Protection header.
    #[builder(into)]
    #[serde(rename = "reportUri")]
    pub r#report_uri: Box<String>,
}
