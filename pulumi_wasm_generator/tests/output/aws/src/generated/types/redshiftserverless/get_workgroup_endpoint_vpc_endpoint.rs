#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetWorkgroupEndpointVpcEndpoint {
    /// The network interfaces of the endpoint.. See `Network Interface` below.
    #[builder(into)]
    #[serde(rename = "networkInterfaces")]
    pub r#network_interfaces: Box<Vec<super::super::types::redshiftserverless::GetWorkgroupEndpointVpcEndpointNetworkInterface>>,
    /// The DNS address of the VPC endpoint.
    #[builder(into)]
    #[serde(rename = "vpcEndpointId")]
    pub r#vpc_endpoint_id: Box<String>,
    /// The port that Amazon Redshift Serverless listens on.
    #[builder(into)]
    #[serde(rename = "vpcId")]
    pub r#vpc_id: Box<String>,
}
