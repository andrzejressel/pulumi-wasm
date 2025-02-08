#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod get_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct GetRouteArgs {
        /// EC2 Carrier Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub carrier_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Core network ARN of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub core_network_arn: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// CIDR block of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub destination_cidr_block: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// IPv6 CIDR block of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub destination_ipv6_cidr_block: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// ID of a managed prefix list destination of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub destination_prefix_list_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
        /// Egress Only Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub egress_only_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Instance ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub instance_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Local Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub local_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// NAT Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub nat_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Network Interface ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub network_interface_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// ID of the specific Route Table containing the Route entry.
        ///
        /// The following arguments are optional:
        #[builder(into)]
        pub route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// EC2 Transit Gateway ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// VPC Peering Connection ID of the Route belonging to the Route Table.
        #[builder(into, default)]
        pub vpc_peering_connection_id: pulumi_gestalt_rust::InputOrOutput<
            Option<String>,
        >,
    }
    #[allow(dead_code)]
    pub struct GetRouteResult {
        pub carrier_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub core_network_arn: pulumi_gestalt_rust::Output<String>,
        pub destination_cidr_block: pulumi_gestalt_rust::Output<String>,
        pub destination_ipv6_cidr_block: pulumi_gestalt_rust::Output<String>,
        pub destination_prefix_list_id: pulumi_gestalt_rust::Output<String>,
        pub egress_only_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub gateway_id: pulumi_gestalt_rust::Output<String>,
        /// The provider-assigned unique ID for this managed resource.
        pub id: pulumi_gestalt_rust::Output<String>,
        pub instance_id: pulumi_gestalt_rust::Output<String>,
        pub local_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub nat_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub network_interface_id: pulumi_gestalt_rust::Output<String>,
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
        pub transit_gateway_id: pulumi_gestalt_rust::Output<String>,
        pub vpc_peering_connection_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn invoke(
        context: &pulumi_gestalt_rust::PulumiContext,
        args: GetRouteArgs,
    ) -> GetRouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let carrier_gateway_id_binding = args
            .carrier_gateway_id
            .get_output(context)
            .get_inner();
        let core_network_arn_binding = args
            .core_network_arn
            .get_output(context)
            .get_inner();
        let destination_cidr_block_binding = args
            .destination_cidr_block
            .get_output(context)
            .get_inner();
        let destination_ipv6_cidr_block_binding = args
            .destination_ipv6_cidr_block
            .get_output(context)
            .get_inner();
        let destination_prefix_list_id_binding = args
            .destination_prefix_list_id
            .get_output(context)
            .get_inner();
        let egress_only_gateway_id_binding = args
            .egress_only_gateway_id
            .get_output(context)
            .get_inner();
        let gateway_id_binding = args.gateway_id.get_output(context).get_inner();
        let instance_id_binding = args.instance_id.get_output(context).get_inner();
        let local_gateway_id_binding = args
            .local_gateway_id
            .get_output(context)
            .get_inner();
        let nat_gateway_id_binding = args.nat_gateway_id.get_output(context).get_inner();
        let network_interface_id_binding = args
            .network_interface_id
            .get_output(context)
            .get_inner();
        let route_table_id_binding = args.route_table_id.get_output(context).get_inner();
        let transit_gateway_id_binding = args
            .transit_gateway_id
            .get_output(context)
            .get_inner();
        let vpc_peering_connection_id_binding = args
            .vpc_peering_connection_id
            .get_output(context)
            .get_inner();
        let request = register_interface::ResourceInvokeRequest {
            token: "aws:ec2/getRoute:getRoute".into(),
            version: super::super::super::get_version(),
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
        };
        let o = register_interface::invoke(context.get_inner(), &request);
        GetRouteResult {
            carrier_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("carrierGatewayId"),
            ),
            core_network_arn: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("coreNetworkArn"),
            ),
            destination_cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationCidrBlock"),
            ),
            destination_ipv6_cidr_block: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationIpv6CidrBlock"),
            ),
            destination_prefix_list_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("destinationPrefixListId"),
            ),
            egress_only_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("egressOnlyGatewayId"),
            ),
            gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("gatewayId"),
            ),
            id: pulumi_gestalt_rust::__private::into_domain(o.extract_field("id")),
            instance_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("instanceId"),
            ),
            local_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("localGatewayId"),
            ),
            nat_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("natGatewayId"),
            ),
            network_interface_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("networkInterfaceId"),
            ),
            route_table_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("routeTableId"),
            ),
            transit_gateway_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("transitGatewayId"),
            ),
            vpc_peering_connection_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("vpcPeeringConnectionId"),
            ),
        }
    }
}
