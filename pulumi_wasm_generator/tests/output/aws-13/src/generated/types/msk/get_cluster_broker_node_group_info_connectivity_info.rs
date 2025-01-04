#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterBrokerNodeGroupInfoConnectivityInfo {
    #[builder(into)]
    #[serde(rename = "publicAccesses")]
    pub r#public_accesses: Box<Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoPublicAccess>>,
    #[builder(into)]
    #[serde(rename = "vpcConnectivities")]
    pub r#vpc_connectivities: Box<Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivity>>,
}
