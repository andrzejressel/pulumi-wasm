/// Manages an EC2 Local Gateway Route. More information can be found in the [Outposts User Guide](https://docs.aws.amazon.com/outposts/latest/userguide/outposts-networking-components.html#routing).
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = local_gateway_route::create(
///         "example",
///         LocalGatewayRouteArgs::builder()
///             .destination_cidr_block("172.16.0.0/16")
///             .local_gateway_route_table_id("${exampleAwsEc2LocalGatewayRouteTable.id}")
///             .local_gateway_virtual_interface_group_id(
///                 "${exampleAwsEc2LocalGatewayVirtualInterfaceGroup.id}",
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Using `pulumi import`, import `aws_ec2_local_gateway_route` using the EC2 Local Gateway Route Table identifier and destination CIDR block separated by underscores (`_`). For example:
///
/// ```sh
/// $ pulumi import aws:ec2/localGatewayRoute:LocalGatewayRoute example lgw-rtb-12345678_172.16.0.0/16
/// ```
pub mod local_gateway_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct LocalGatewayRouteArgs {
        /// IPv4 CIDR range used for destination matches. Routing decisions are based on the most specific match.
        #[builder(into)]
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Local Gateway Route Table.
        #[builder(into)]
        pub local_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Local Gateway Virtual Interface Group.
        #[builder(into)]
        pub local_gateway_virtual_interface_group_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct LocalGatewayRouteResult {
        /// IPv4 CIDR range used for destination matches. Routing decisions are based on the most specific match.
        pub destination_cidr_block: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Local Gateway Route Table.
        pub local_gateway_route_table_id: pulumi_wasm_rust::Output<String>,
        /// Identifier of EC2 Local Gateway Virtual Interface Group.
        pub local_gateway_virtual_interface_group_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: LocalGatewayRouteArgs) -> LocalGatewayRouteResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destination_cidr_block_binding = args.destination_cidr_block.get_inner();
        let local_gateway_route_table_id_binding = args
            .local_gateway_route_table_id
            .get_inner();
        let local_gateway_virtual_interface_group_id_binding = args
            .local_gateway_virtual_interface_group_id
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "aws:ec2/localGatewayRoute:LocalGatewayRoute".into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinationCidrBlock".into(),
                    value: &destination_cidr_block_binding,
                },
                register_interface::ObjectField {
                    name: "localGatewayRouteTableId".into(),
                    value: &local_gateway_route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "localGatewayVirtualInterfaceGroupId".into(),
                    value: &local_gateway_virtual_interface_group_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "destinationCidrBlock".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayRouteTableId".into(),
                },
                register_interface::ResultField {
                    name: "localGatewayVirtualInterfaceGroupId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        LocalGatewayRouteResult {
            destination_cidr_block: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("destinationCidrBlock").unwrap(),
            ),
            local_gateway_route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayRouteTableId").unwrap(),
            ),
            local_gateway_virtual_interface_group_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("localGatewayVirtualInterfaceGroupId").unwrap(),
            ),
        }
    }
}