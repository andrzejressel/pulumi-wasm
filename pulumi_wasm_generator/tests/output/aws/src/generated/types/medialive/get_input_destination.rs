#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetInputDestination {
    #[builder(into)]
    #[serde(rename = "ip")]
    pub r#ip: Box<String>,
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<String>,
    #[builder(into)]
    #[serde(rename = "url")]
    pub r#url: Box<String>,
    #[builder(into)]
    #[serde(rename = "vpcs")]
    pub r#vpcs: Box<Vec<super::super::types::medialive::GetInputDestinationVpc>>,
}