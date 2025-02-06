#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthentication {
    /// Configuration block for specifying SASL client authentication. See below.
    #[builder(into, default)]
    #[serde(rename = "sasl")]
    pub r#sasl: Box<Option<super::super::types::msk::ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl>>,
    /// Configuration block for specifying TLS client authentication. See below.
    #[builder(into, default)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<bool>>,
}
