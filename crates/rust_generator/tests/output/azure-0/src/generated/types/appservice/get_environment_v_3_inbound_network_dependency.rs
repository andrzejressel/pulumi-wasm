#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentV3InboundNetworkDependency {
    /// A short description of the purpose of the network traffic.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// A list of IP addresses that network traffic will originate from in CIDR notation.
    #[builder(into)]
    #[serde(rename = "ipAddresses")]
    pub r#ip_addresses: Box<Vec<String>>,
    /// The ports that network traffic will arrive to the App Service Environment V3 on.
    #[builder(into)]
    #[serde(rename = "ports")]
    pub r#ports: Box<Vec<String>>,
}
