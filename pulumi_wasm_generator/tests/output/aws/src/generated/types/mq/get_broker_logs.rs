#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBrokerLogs {
    #[builder(into)]
    #[serde(rename = "audit")]
    pub r#audit: Box<bool>,
    #[builder(into)]
    #[serde(rename = "general")]
    pub r#general: Box<bool>,
}