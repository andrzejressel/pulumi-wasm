#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponseHeadersPolicySecurityHeadersConfigStrictTransportSecurity {
    /// A number that CloudFront uses as the value for the `max-age` directive in the `Strict-Transport-Security` HTTP response header.
    #[builder(into)]
    #[serde(rename = "accessControlMaxAgeSec")]
    pub r#access_control_max_age_sec: Box<i32>,
    /// Whether CloudFront includes the `includeSubDomains` directive in the `Strict-Transport-Security` HTTP response header.
    #[builder(into, default)]
    #[serde(rename = "includeSubdomains")]
    pub r#include_subdomains: Box<Option<bool>>,
    /// Whether CloudFront overrides the `Strict-Transport-Security` HTTP response header received from the origin with the one specified in this response headers policy.
    #[builder(into)]
    #[serde(rename = "override")]
    pub r#override_: Box<bool>,
    /// Whether CloudFront includes the `preload` directive in the `Strict-Transport-Security` HTTP response header.
    #[builder(into, default)]
    #[serde(rename = "preload")]
    pub r#preload: Box<Option<bool>>,
}
