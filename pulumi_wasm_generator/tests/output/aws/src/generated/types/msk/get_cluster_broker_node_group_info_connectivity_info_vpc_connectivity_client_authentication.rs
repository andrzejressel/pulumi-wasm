#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication {
    #[builder(into)]
    #[serde(rename = "sasls")]
    pub r#sasls: Box<Vec<super::super::types::msk::GetClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl>>,
    #[builder(into)]
    #[serde(rename = "tls")]
    pub r#tls: Box<bool>,
}