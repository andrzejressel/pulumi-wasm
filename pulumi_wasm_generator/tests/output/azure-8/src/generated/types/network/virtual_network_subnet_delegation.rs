#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkSubnetDelegation {
    /// A name for this delegation.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `service_delegation` block as defined below.
    #[builder(into)]
    #[serde(rename = "serviceDelegation")]
    pub r#service_delegation: Box<super::super::types::network::VirtualNetworkSubnetDelegationServiceDelegation>,
}
