#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterBrokerNodeGroupInfoConnectivityInfoVpcConnectivityClientAuthenticationSasl {
    #[builder(into, default)]
    #[serde(rename = "iam")]
    pub r#iam: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "scram")]
    pub r#scram: Box<Option<bool>>,
}