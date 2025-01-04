#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVpcPeeringConnectionPeerIpv6CidrBlockSet {
    #[builder(into)]
    #[serde(rename = "ipv6CidrBlock")]
    pub r#ipv_6_cidr_block: Box<String>,
}
