#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOriginRequestPolicyCookiesConfig {
    #[builder(into)]
    #[serde(rename = "cookieBehavior")]
    pub r#cookie_behavior: Box<String>,
    #[builder(into)]
    #[serde(rename = "cookies")]
    pub r#cookies: Box<Vec<super::super::types::cloudfront::GetOriginRequestPolicyCookiesConfigCookie>>,
}
