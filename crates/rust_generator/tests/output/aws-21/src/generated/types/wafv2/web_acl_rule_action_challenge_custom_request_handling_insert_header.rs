#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WebAclRuleActionChallengeCustomRequestHandlingInsertHeader {
    /// Name of the custom header. For custom request header insertion, when AWS WAF inserts the header into the request, it prefixes this name `x-amzn-waf-`, to avoid confusion with the headers that are already in the request. For example, for the header name `sample`, AWS WAF inserts the header `x-amzn-waf-sample`.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Value of the custom header.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
