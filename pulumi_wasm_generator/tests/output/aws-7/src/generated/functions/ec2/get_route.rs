pub mod get_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteArgs {
        /// EC2 Carrier Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub carrier_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Core network ARN of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub core_network_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// CIDR block of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub destination_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// IPv6 CIDR block of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub destination_ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of a managed prefix list destination of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub destination_prefix_list_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Egress Only Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub egress_only_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Instance ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub instance_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Local Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub local_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// NAT Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub nat_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Network Interface ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub network_interface_id: pulumi_wasm_rust::Output<Option<String>>,
        /// ID of the specific Route Table containing the Route entry.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// EC2 Transit Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// VPC Peering Connection ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub vpc_peering_connection_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct GetRouteResult {
        pub carrier_gateway_id: pulumi_wasm_rust::Output<String>,
        pub core_network_arn: pulumi_wasm_rust::Output<String>,
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        pub destination_ipv6_cidr_block: pulumi_wasm_rust::Output<String>,
        pub destination_prefix_list_id: pulumi_wasm_rust::Output<String>,
        pub egress_only_gateway_id: pulumi_wasm_rust::Output<String>,
        pub gateway_id: pulumi_wasm_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_wasm_rust::Output<String>,
        pub instance_id: pulumi_wasm_rust::Output<String>,
        pub local_gateway_id: pulumi_wasm_rust::Output<String>,
        pub nat_gateway_id: pulumi_wasm_rust::Output<String>,
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        pub transit_gateway_id: pulumi_wasm_rust::Output<String>,
        pub vpc_peering_connection_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(args: GetRouteArgs) -> GetRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let carrier_gateway_id_binding = args.carrier_gateway_id.get_inner();
        let core_network_arn_binding = args.core_network_arn.get_inner();
        let destination_cidr_block_binding = args.destination_cidr_block.get_inner();
        let destination_ipv6_cidr_block_binding = args
            .destination_ipv6_cidr_block
            .get_inner();
        let destination_prefix_list_id_binding = args
            .destination_prefix_list_id
            .get_inner();
        let egress_only_gateway_id_binding = args.egress_only_gateway_id.get_inner();
        let gateway_id_binding = args.gateway_id.get_inner();
        let instance_id_binding = args.instance_id.get_inner();
        let local_gateway_id_binding = args.local_gateway_id.get_inner();
        let nat_gateway_id_binding = args.nat_gateway_id.get_inner();
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let route_table_id_binding = args.route_table_id.get_inner();
        let transit_gateway_id_binding = args.transit_gateway_id.get_inner();
        let vpc_peering_connection_id_binding = args
            .vpc_peering_connection_id
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getRoute:getRoute".into(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "carrierGatewayId".into(),
                    value: &carrier_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "coreNetworkArn".into(),
                    value: &core_network_arn_binding,
                },
                register_interface::ObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "destinationIpv6CidrBlock".into(),
                    value: &destination_ipv6_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "destinationPrefixListId".into(),
                    value: &destination_prefix_list_id_binding,
                },
                register_interface::ObjectField {
                    name: "egressOnlyGatewayId".into(),
                    value: &egress_only_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "gatewayId".into(),
                    value: &gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "instanceId".into(),
                    value: &instance_id_binding,
                },
                register_interface::ObjectField {
                    name: "localGatewayId".into(),
                    value: &local_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "natGatewayId".into(),
                    value: &nat_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "networkInterfaceId".into(),
                    value: &network_interface_id_binding,
                },
                register_interface::ObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "transitGatewayId".into(),
                    value: &transit_gateway_id_binding,
                },
                register_interface::ObjectField {
                    name: "vpcPeeringConnectionId".into(),
                    value: &vpc_peering_connection_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "carrierGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "coreNetworkArn".into(),
                },
                register_interface::ResultField {
                    name: "destinationCidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "destinationIpv6CidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "destinationPrefixListId".into(),
                },
                register_interface::ResultField {
                    name: "egressOnlyGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "gatewayId".into(),
                },
                register_interface::ResultField {
                    name: "id".into(),
                },
                register_interface::ResultField {
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "natGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "networkInterfaceId".into(),
                },
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpcPeeringConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::invoke(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        GetRouteResult {
            carrier_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("carrierGatewayId").unwrap(),
            ),
            core_network_arn: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("coreNetworkArn").unwrap(),
            ),
            destination_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationCidrBlock").unwrap(),
            ),
            destination_ipv6_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationIpv6CidrBlock").unwrap(),
            ),
            destination_prefix_list_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationPrefixListId").unwrap(),
            ),
            egress_only_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("egressOnlyGatewayId").unwrap(),
            ),
            gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("gatewayId").unwrap(),
            ),
            id: pulumi_wasm_rust::__private::into_domain(hashmap.remove("id").unwrap()),
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            local_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayId").unwrap(),
            ),
            nat_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("natGatewayId").unwrap(),
            ),
            network_interface_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("networkInterfaceId").unwrap(),
            ),
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            vpc_peering_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcPeeringConnectionId").unwrap(),
            ),
        }
    }
}
