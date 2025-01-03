#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetClusterRemoteNetworkConfigRemoteNodeNetwork {
    /// List of network CIDRs that can contain pods that run Kubernetes webhooks on hybrid nodes.
    #[builder(into)]
    #[serde(rename = "cidrs")]
    pub r#cidrs: Box<Vec<String>>,
}
