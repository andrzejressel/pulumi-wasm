#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TopicRuleErrorActionHttpHttpHeader {
    /// The name of the HTTP header.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The value of the HTTP header.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
