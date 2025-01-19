/// Requests automatic route propagation between a VPN gateway and a route table.
///
/// > **Note:** This resource should not be used with a route table that has
/// the `propagating_vgws` argument set. If that argument is set, any route
/// propagation not explicitly listed in its value will be removed.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = vpn_gateway_route_propagation::create(
///         "example",
///         VpnGatewayRoutePropagationArgs::builder()
///             .route_table_id("${exampleAwsRouteTable.id}")
///             .vpn_gateway_id("${exampleAwsVpnGateway.id}")
///             .build_struct(),
///     );
/// }
/// ```
pub mod vpn_gateway_route_propagation {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VpnGatewayRoutePropagationArgs {
        /// The id of the `aws.ec2.RouteTable` to propagate routes into.
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The id of the `aws.ec2.VpnGateway` to propagate routes from.
        #[builder(into)]
        pub vpn_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct VpnGatewayRoutePropagationResult {
        /// The id of the `aws.ec2.RouteTable` to propagate routes into.
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The id of the `aws.ec2.VpnGateway` to propagate routes from.
        pub vpn_gateway_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: VpnGatewayRoutePropagationArgs,
    ) -> VpnGatewayRoutePropagationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let route_table_id_binding = args.route_table_id.get_inner();
        let vpn_gateway_id_binding = args.vpn_gateway_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/vpnGatewayRoutePropagation:VpnGatewayRoutePropagation"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpnGatewayId".into(),
                    value: &vpn_gateway_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "vpnGatewayId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        VpnGatewayRoutePropagationResult {
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            vpn_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpnGatewayId").unwrap(),
            ),
        }
    }
}
