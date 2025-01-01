#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDatabaseInstanceIpAddress {
    #[builder(into)]
    #[serde(rename = "ipAddress")]
    pub r#ip_address: Box<String>,
    #[builder(into)]
    #[serde(rename = "timeToRetire")]
    pub r#time_to_retire: Box<String>,
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}
