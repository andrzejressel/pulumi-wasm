/// Provides a resource to create a routing table entry (a route) in a VPC routing table.
///
/// > **NOTE on Route Tables and Routes:** This provider currently provides both a standalone Route resource and a Route Table resource with routes defined in-line. At this time you cannot use a Route Table with in-line routes in conjunction with any Route resources. Doing so will cause a conflict of rule settings and will overwrite rules.
///
/// > **NOTE on `gateway_id` attribute:** The AWS API is very forgiving with the resource ID passed in the `gateway_id` attribute. For example an `aws.ec2.Route` resource can be created with an `aws.ec2.NatGateway` or `aws.ec2.EgressOnlyInternetGateway` ID specified for the `gateway_id` attribute. Specifying anything other than an `aws.ec2.InternetGateway` or `aws.ec2.VpnGateway` ID will lead to this provider reporting a permanent diff between your configuration and recorded state, as the AWS API returns the more-specific attribute. If you are experiencing constant diffs with an `aws.ec2.Route` resource, the first thing to check is that the correct attribute is being specified.
///
/// > **NOTE on combining `vpc_endpoint_id` and `destination_prefix_list_id` attributes:** To associate a Gateway VPC Endpoint (such as S3) with destination prefix list, use the `aws.ec2.VpcEndpointRouteTableAssociation` resource instead.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let r = route::create(
///         "r",
///         RouteArgs::builder()
///             .destination_cidr_block("10.0.1.0/22")
///             .route_table_id("${testing.id}")
///             .vpc_peering_connection_id("pcx-45ff3dc1")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Example IPv6 Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let egress = egress_only_internet_gateway::create(
///         "egress",
///         EgressOnlyInternetGatewayArgs::builder().vpc_id("${vpc.id}").build_struct(),
///     );
///     let r = route::create(
///         "r",
///         RouteArgs::builder()
///             .destination_ipv_6_cidr_block("::/0")
///             .egress_only_gateway_id("${egress.id}")
///             .route_table_id("rtb-4fbb3ac4")
///             .build_struct(),
///     );
///     let vpc = vpc::create(
///         "vpc",
///         VpcArgs::builder()
///             .assign_generated_ipv_6_cidr_block(true)
///             .cidr_block("10.1.0.0/16")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Import a route in route table `rtb-656C65616E6F72` with an IPv6 destination CIDR of `2620:0:2d0:200::8/125`:
///
/// Import a route in route table `rtb-656C65616E6F72` with a managed prefix list destination of `pl-0570a1d2d725c16be`:
///
/// __Using `pulumi import` to import__ individual routes using `ROUTETABLEID_DESTINATION`. Import [local routes](https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Route_Tables.html#RouteTables) using the VPC's IPv4 or IPv6 CIDR blocks. For example:
///
/// Import a route in route table `rtb-656C65616E6F72` with an IPv4 destination CIDR of `10.42.0.0/16`:
///
/// ```sh
/// $ pulumi import aws:ec2/route:Route my_route rtb-656C65616E6F72_10.42.0.0/16
/// ```
/// Import a route in route table `rtb-656C65616E6F72` with an IPv6 destination CIDR of `2620:0:2d0:200::8/125`:
///
/// ```sh
/// $ pulumi import aws:ec2/route:Route my_route rtb-656C65616E6F72_2620:0:2d0:200::8/125
/// ```
/// Import a route in route table `rtb-656C65616E6F72` with a managed prefix list destination of `pl-0570a1d2d725c16be`:
///
/// ```sh
/// $ pulumi import aws:ec2/route:Route my_route rtb-656C65616E6F72_pl-0570a1d2d725c16be
/// ```
pub mod route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// Identifier of a carrier gateway. This attribute can only be used when the VPC contains a subnet which is associated with a Wavelength Zone.
        #[builder(into, default)]
        pub carrier_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of a core network.
        #[builder(into, default)]
        pub core_network_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The destination CIDR block.
        #[builder(into, default)]
        pub destination_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The destination IPv6 CIDR block.
        #[builder(into, default)]
        pub destination_ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of a managed prefix list destination.
        ///
        /// One of the following target arguments must be supplied:
        #[builder(into, default)]
        pub destination_prefix_list_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC Egress Only Internet Gateway.
        #[builder(into, default)]
        pub egress_only_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC internet gateway or a virtual private gateway. Specify `local` when updating a previously imported local route.
        #[builder(into, default)]
        pub gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a Outpost local gateway.
        #[builder(into, default)]
        pub local_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC NAT gateway.
        #[builder(into, default)]
        pub nat_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of an EC2 network interface.
        #[builder(into, default)]
        pub network_interface_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the routing table.
        ///
        /// One of the following destination arguments must be supplied:
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of an EC2 Transit Gateway.
        #[builder(into, default)]
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC Endpoint.
        #[builder(into, default)]
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC peering connection.
        ///
        /// Note that the default route, mapping the VPC's CIDR block to "local", is created implicitly and cannot be specified.
        #[builder(into, default)]
        pub vpc_peering_connection_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// Identifier of a carrier gateway. This attribute can only be used when the VPC contains a subnet which is associated with a Wavelength Zone.
        pub carrier_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// The Amazon Resource Name (ARN) of a core network.
        pub core_network_arn: pulumi_wasm_rust::Output<Option<String>>,
        /// The destination CIDR block.
        pub destination_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The destination IPv6 CIDR block.
        pub destination_ipv6_cidr_block: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of a managed prefix list destination.
        ///
        /// One of the following target arguments must be supplied:
        pub destination_prefix_list_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC Egress Only Internet Gateway.
        pub egress_only_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC internet gateway or a virtual private gateway. Specify `local` when updating a previously imported local route.
        pub gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of an EC2 instance.
        pub instance_id: pulumi_wasm_rust::Output<String>,
        /// The AWS account ID of the owner of the EC2 instance.
        pub instance_owner_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of a Outpost local gateway.
        pub local_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC NAT gateway.
        pub nat_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of an EC2 network interface.
        pub network_interface_id: pulumi_wasm_rust::Output<String>,
        /// How the route was created - `CreateRouteTable`, `CreateRoute` or `EnableVgwRoutePropagation`.
        pub origin: pulumi_wasm_rust::Output<String>,
        /// The ID of the routing table.
        ///
        /// One of the following destination arguments must be supplied:
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The state of the route - `active` or `blackhole`.
        pub state: pulumi_wasm_rust::Output<String>,
        /// Identifier of an EC2 Transit Gateway.
        pub transit_gateway_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC Endpoint.
        pub vpc_endpoint_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Identifier of a VPC peering connection.
        ///
        /// Note that the default route, mapping the VPC's CIDR block to "local", is created implicitly and cannot be specified.
        pub vpc_peering_connection_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: RouteArgs) -> RouteResult {
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
        let local_gateway_id_binding = args.local_gateway_id.get_inner();
        let nat_gateway_id_binding = args.nat_gateway_id.get_inner();
        let network_interface_id_binding = args.network_interface_id.get_inner();
        let route_table_id_binding = args.route_table_id.get_inner();
        let transit_gateway_id_binding = args.transit_gateway_id.get_inner();
        let vpc_endpoint_id_binding = args.vpc_endpoint_id.get_inner();
        let vpc_peering_connection_id_binding = args
            .vpc_peering_connection_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/route:Route".into(),
            name: name.to_string(),
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
                    name: "vpcEndpointId".into(),
                    value: &vpc_endpoint_id_binding,
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
                    name: "instanceId".into(),
                },
                register_interface::ResultField {
                    name: "instanceOwnerId".into(),
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
                    name: "origin".into(),
                },
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "state".into(),
                },
                register_interface::ResultField {
                    name: "transitGatewayId".into(),
                },
                register_interface::ResultField {
                    name: "vpcEndpointId".into(),
                },
                register_interface::ResultField {
                    name: "vpcPeeringConnectionId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RouteResult {
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
            instance_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceId").unwrap(),
            ),
            instance_owner_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("instanceOwnerId").unwrap(),
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
            origin: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("origin").unwrap(),
            ),
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            state: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("state").unwrap(),
            ),
            transit_gateway_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("transitGatewayId").unwrap(),
            ),
            vpc_endpoint_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcEndpointId").unwrap(),
            ),
            vpc_peering_connection_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("vpcPeeringConnectionId").unwrap(),
            ),
        }
    }
}