#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ResponseHeadersPolicySecurityHeadersConfigFrameOptions {
    /// The value of the `X-Frame-Options` HTTP response header. Valid values: `DENY` | `SAMEORIGIN`
    #[builder(into)]
    #[serde(rename = "frameOption")]
    pub r#frame_option: Box<String>,
    /// Whether CloudFront overrides the `X-Frame-Options` HTTP response header received from the origin with the one specified in this response headers policy.
    #[builder(into)]
    #[serde(rename = "override")]
    pub r#override_: Box<bool>,
}
