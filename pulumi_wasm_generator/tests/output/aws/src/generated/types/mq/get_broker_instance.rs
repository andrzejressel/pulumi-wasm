#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetBrokerInstance {
    #[builder(into)]
    #[serde(rename = "consoleUrl")]
    pub r#console_url: Box<String>,
    #[builder(into)]
    #[serde(rename = "endpoints")]
    pub r#endpoints: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
}
