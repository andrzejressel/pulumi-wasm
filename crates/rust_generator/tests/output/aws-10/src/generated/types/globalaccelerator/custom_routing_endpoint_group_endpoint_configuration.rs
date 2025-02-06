#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CustomRoutingEndpointGroupEndpointConfiguration {
    /// An ID for the endpoint. For custom routing accelerators, this is the virtual private cloud (VPC) subnet ID.
    #[builder(into, default)]
    #[serde(rename = "endpointId")]
    pub r#endpoint_id: Box<Option<String>>,
}
