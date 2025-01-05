#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetGatewayConnectionTrafficSelectorPolicy {
    /// List of local CIDRs.
    #[builder(into)]
    #[serde(rename = "localAddressCidrs")]
    pub r#local_address_cidrs: Box<Vec<String>>,
    /// List of remote CIDRs.
    #[builder(into)]
    #[serde(rename = "remoteAddressCidrs")]
    pub r#remote_address_cidrs: Box<Vec<String>>,
}
