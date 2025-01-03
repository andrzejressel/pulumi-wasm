#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOriginRequestPolicyHeadersConfig {
    #[builder(into)]
    #[serde(rename = "headerBehavior")]
    pub r#header_behavior: Box<String>,
    #[builder(into)]
    #[serde(rename = "headers")]
    pub r#headers: Box<Vec<super::super::types::cloudfront::GetOriginRequestPolicyHeadersConfigHeader>>,
}
