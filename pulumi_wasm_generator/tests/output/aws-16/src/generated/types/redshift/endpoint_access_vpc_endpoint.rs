#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EndpointAccessVpcEndpoint {
    /// One or more network interfaces of the endpoint. Also known as an interface endpoint. See details below.
    #[builder(into, default)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Box<Option<Vec<super::super::types::redshift::EndpointAccessVpcEndpointNetworkInterface>>>,
    /// The connection endpoint ID for connecting an Amazon Redshift cluster through the proxy.
    #[builder(into, default)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Box<Option<String>>,
    /// The VPC identifier that the endpoint is associated.
    #[builder(into, default)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<Option<String>>,
}
