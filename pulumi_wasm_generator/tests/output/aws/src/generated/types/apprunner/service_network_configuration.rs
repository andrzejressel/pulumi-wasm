#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ServiceNetworkConfiguration {
    /// Network configuration settings for outbound message traffic. See Egress Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "egressConfiguration")]
    pub r#egress_configuration: Box<Option<super::super::types::apprunner::ServiceNetworkConfigurationEgressConfiguration>>,
    /// Network configuration settings for inbound network traffic. See Ingress Configuration below for more details.
    #[builder(into, default)]
    #[serde(rename = "ingressConfiguration")]
    pub r#ingress_configuration: Box<Option<super::super::types::apprunner::ServiceNetworkConfigurationIngressConfiguration>>,
    /// App Runner provides you with the option to choose between Internet Protocol version 4 (IPv4) and dual stack (IPv4 and IPv6) for your incoming public network configuration. Valid values: `IPV4`, `DUAL_STACK`. Default: `IPV4`.
    #[builder(into, default)]
    #[serde(rename = "ipAddressType")]
    pub r#ip_address_type: Box<Option<String>>,
}
