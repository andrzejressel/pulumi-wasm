#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationGatewayPrivateLinkConfiguration {
    /// The ID of the Rewrite Rule Set
    #[builder(into, default)]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    /// One or more `ip_configuration` blocks as defined below.
    /// 
    /// > **Please Note**: The `AllowApplicationGatewayPrivateLink` feature must be registered on the subscription before enabling private link
    /// 
    /// ```bash
    /// az feature register --name AllowApplicationGatewayPrivateLink --namespace Microsoft.Network
    /// ```
    #[builder(into)]
    #[serde(rename = "ipConfigurations")]
    pub r#ip_configurations: Box<Vec<super::super::types::network::ApplicationGatewayPrivateLinkConfigurationIpConfiguration>>,
    /// The name of the private link configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
