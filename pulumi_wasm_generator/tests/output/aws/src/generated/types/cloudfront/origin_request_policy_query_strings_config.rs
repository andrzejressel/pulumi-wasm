#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct OriginRequestPolicyQueryStringsConfig {
    #[builder(into)]
    #[serde(rename = "queryStringBehavior")]
    pub r#query_string_behavior: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Box<Option<super::super::types::cloudfront::OriginRequestPolicyQueryStringsConfigQueryStrings>>,
}
