#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNetworkSubnetSharedPublicIpAddress {
    /// A list of `allowed_ports` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "allowedPorts")]
    pub r#allowed_ports: Box<Option<Vec<super::super::types::devtest::VirtualNetworkSubnetSharedPublicIpAddressAllowedPort>>>,
}
