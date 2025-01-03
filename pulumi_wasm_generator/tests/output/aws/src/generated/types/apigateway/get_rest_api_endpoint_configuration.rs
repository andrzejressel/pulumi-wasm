#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetRestApiEndpointConfiguration {
    #[builder(into)]
    #[serde(rename = "types")]
    pub r#types: Box<Vec<String>>,
    #[builder(into)]
    #[serde(rename = "vpcEndpointIds")]
    pub r#vpc_endpoint_ids: Box<Vec<String>>,
}
