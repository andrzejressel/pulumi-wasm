#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetTrafficPolicyDocumentRuleItem {
    #[builder(into, default)]
    #[serde(rename = "endpointReference")]
    pub r#endpoint_reference: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "healthCheck")]
    pub r#health_check: Box<Option<String>>,
}