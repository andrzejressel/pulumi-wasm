#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetOriginRequestPolicyQueryStringsConfig {
    #[builder(into)]
    #[serde(rename = "queryStringBehavior")]
    pub r#query_string_behavior: Box<String>,
    #[builder(into)]
    #[serde(rename = "queryStrings")]
    pub r#query_strings: Box<Vec<super::super::types::cloudfront::GetOriginRequestPolicyQueryStringsConfigQueryString>>,
}