#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRouteSpecTcpRouteActionWeightedTarget {
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    #[builder(into)]
    #[serde(rename = "virtualNode")]
    pub r#virtual_node: Box<String>,
    #[builder(into)]
    #[serde(rename = "weight")]
    pub r#weight: Box<i32>,
}