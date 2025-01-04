#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleErrorActionHttp {
    /// The HTTPS URL used to verify ownership of `url`.
    #[builder(into, default)]
    #[serde(rename = "confirmationUrl")]
    pub r#confirmation_url: Box<Option<String>>,
    /// Custom HTTP header IoT Core should send. It is possible to define more than one custom header.
    #[builder(into, default)]
    #[serde(rename = "httpHeaders")]
    pub r#http_headers: Box<Option<Vec<super::super::types::iot::TopicRuleErrorActionHttpHttpHeader>>>,
    /// The HTTPS URL.
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
}
