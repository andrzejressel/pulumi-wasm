#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpcPeeringConnectionCidrBlockSet {
    /// Primary CIDR block of the requester VPC of the specific VPC Peering Connection to retrieve.
    #[builder(into)]
    #[serde(rename = "cidrBlock")]
    pub r#cidr_block: Box<String>,
}