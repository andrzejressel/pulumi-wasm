/// Provides a static route between a VPN connection and a customer gateway.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   vpc:
///     type: aws:ec2:Vpc
///     properties:
///       cidrBlock: 10.0.0.0/16
///   vpnGateway:
///     type: aws:ec2:VpnGateway
///     name: vpn_gateway
///     properties:
///       vpcId: ${vpc.id}
///   customerGateway:
///     type: aws:ec2:CustomerGateway
///     name: customer_gateway
///     properties:
///       bgpAsn: 65000
///       ipAddress: 172.0.0.1
///       type: ipsec.1
///   main:
///     type: aws:ec2:VpnConnection
///     properties:
///       vpnGatewayId: ${vpnGateway.id}
///       customerGatewayId: ${customerGateway.id}
///       type: ipsec.1
///       staticRoutesOnly: true
///   office:
///     type: aws:ec2:VpnConnectionRoute
///     properties:
///       destinationCidrBlock: 192.168.10.0/24
///       vpnConnectionId: ${main.id}
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod vpn_connection_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnConnectionRouteArgs {
        /// The CIDR block associated with the local subnet of the customer network.
        #[builder(into)]
        pub destination_cidr_block: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The ID of the VPN connection.
        #[builder(into)]
        pub vpn_connection_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VpnConnectionRouteResult {
        /// The CIDR block associated with the local subnet of the customer network.
        pub destination_cidr_block: pulumi_gestalt_rust::Output<String>,
        /// The ID of the VPN connection.
        pub vpn_connection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VpnConnectionRouteArgs,
    ) -> VpnConnectionRouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destination_cidr_block_binding = args
            .destination_cidr_block
            .get_output(context);
        let vpn_connection_id_binding = args.vpn_connection_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "aws:ec2/vpnConnectionRoute:VpnConnectionRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationCidrBlock".into(),
                    value: destination_cidr_block_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "vpnConnectionId".into(),
                    value: vpn_connection_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VpnConnectionRouteResult {
            destination_cidr_block: o.get_field("destinationCidrBlock"),
            vpn_connection_id: o.get_field("vpnConnectionId"),
        }
    }
}
