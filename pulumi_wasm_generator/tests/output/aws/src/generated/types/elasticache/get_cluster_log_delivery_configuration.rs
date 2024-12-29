#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterLogDeliveryConfiguration {
    #[builder(into)]
    #[serde(rename = "destination")]
    pub r#destination: Box<String>,
    #[builder(into)]
    #[serde(rename = "destinationType")]
    pub r#destination_type: Box<String>,
    #[builder(into)]
    #[serde(rename = "logFormat")]
    pub r#log_format: Box<String>,
    #[builder(into)]
    #[serde(rename = "logType")]
    pub r#log_type: Box<String>,
}
