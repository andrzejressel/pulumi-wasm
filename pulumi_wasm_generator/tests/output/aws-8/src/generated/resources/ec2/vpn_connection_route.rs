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
pub mod vpn_connection_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnConnectionRouteArgs {
        /// The CIDR block associated with the local subnet of the customer network.
        #[builder(into)]
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPN connection.
        #[builder(into)]
        pub vpn_connection_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpnConnectionRouteResult {
        /// The CIDR block associated with the local subnet of the customer network.
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// The ID of the VPN connection.
        pub vpn_connection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: VpnConnectionRouteArgs) -> VpnConnectionRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_cidr_block_binding = args.destination_cidr_block.get_inner();
        let vpn_connection_id_binding = args.vpn_connection_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpnConnectionRoute:VpnConnectionRoute".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "vpnConnectionId".into(),
                    value: &vpn_connection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationCidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "vpnConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpnConnectionRouteResult {
            destination_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationCidrBlock").unwrap(),
            ),
            vpn_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnConnectionId").unwrap(),
            ),
        }
    }
}
