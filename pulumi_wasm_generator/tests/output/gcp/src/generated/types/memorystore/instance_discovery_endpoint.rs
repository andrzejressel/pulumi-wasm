#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceDiscoveryEndpoint {
    /// (Output)
    /// Output only. IP address of the exposed endpoint clients connect to.
    #[builder(into, default)]
    #[serde(rename = "address")]
    pub r#address: Box<Option<String>>,
    /// (Output)
    /// Output only. The consumer network where the IP address resides, in the form of
    /// projects/{project_id}/global/networks/{network_id}.
    #[builder(into, default)]
    #[serde(rename = "network")]
    pub r#network: Box<Option<String>>,
    /// (Output)
    /// Output only. Ports of the exposed endpoint.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
}
