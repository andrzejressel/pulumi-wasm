/// Associates a Route Table with a Subnet within a Virtual Network.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = resource_group::create(
///         "example",
///         ResourceGroupArgs::builder()
///             .location("West Europe")
///             .name("example-resources")
///             .build_struct(),
///     );
///     let exampleRouteTable = route_table::create(
///         "exampleRouteTable",
///         RouteTableArgs::builder()
///             .location("${example.location}")
///             .name("example-routetable")
///             .resource_group_name("${example.name}")
///             .routes(
///                 vec![
///                     RouteTableRoute::builder().addressPrefix("10.100.0.0/14")
///                     .name("example").nextHopInIpAddress("10.10.1.1")
///                     .nextHopType("VirtualAppliance").build_struct(),
///                 ],
///             )
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.0.2.0/24",])
///             .name("frontend")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleSubnetRouteTableAssociation = subnet_route_table_association::create(
///         "exampleSubnetRouteTableAssociation",
///         SubnetRouteTableAssociationArgs::builder()
///             .route_table_id("${exampleRouteTable.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.0.0.0/16",])
///             .location("${example.location}")
///             .name("example-network")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Subnet Route Table Associations can be imported using the `resource id` of the Subnet, e.g.
///
/// ```sh
/// $ pulumi import azure:network/subnetRouteTableAssociation:SubnetRouteTableAssociation association1 /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/virtualNetworks/myvnet1/subnets/mysubnet1
/// ```
///
pub mod subnet_route_table_association {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct SubnetRouteTableAssociationArgs {
        /// The ID of the Route Table which should be associated with the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        #[builder(into)]
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct SubnetRouteTableAssociationResult {
        /// The ID of the Route Table which should be associated with the Subnet. Changing this forces a new resource to be created.
        pub route_table_id: pulumi_wasm_rust::Output<String>,
        /// The ID of the Subnet. Changing this forces a new resource to be created.
        pub subnet_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: SubnetRouteTableAssociationArgs,
    ) -> SubnetRouteTableAssociationResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let route_table_id_binding = args.route_table_id.get_inner();
        let subnet_id_binding = args.subnet_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/subnetRouteTableAssociation:SubnetRouteTableAssociation"
                .into(),
            name: name.to_string(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding,
                },
                register_interface::ObjectField {
                    name: "subnetId".into(),
                    value: &subnet_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "routeTableId".into(),
                },
                register_interface::ResultField {
                    name: "subnetId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        SubnetRouteTableAssociationResult {
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("routeTableId").unwrap(),
            ),
            subnet_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("subnetId").unwrap(),
            ),
        }
    }
}