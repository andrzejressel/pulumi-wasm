#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct KxEnvironmentTransitGatewayConfiguration {
    /// Rules that define how you manage outbound traffic from kdb network to your internal network. Defined below.
    #[builder(into, default)]
    #[serde(rename = "attachmentNetworkAclConfigurations")]
    pub r#attachment_network_acl_configurations: Box<Option<Vec<super::super::types::finspace::KxEnvironmentTransitGatewayConfigurationAttachmentNetworkAclConfiguration>>>,
    /// Routing CIDR on behalf of KX environment. It could be any “/26 range in the 100.64.0.0 CIDR space. After providing, it will be added to the customer’s transit gateway routing table so that the traffics could be routed to KX network.
    #[builder(into)]
    #[serde(rename = "routableCidrSpace")]
    pub r#routable_cidr_space: Box<String>,
    /// Identifier of the transit gateway created by the customer to connect outbound traffics from KX network to your internal network.
    #[builder(into)]
    #[serde(rename = "transitGatewayId")]
    pub r#transit_gateway_id: Box<String>,
}
