/// Manages a Route in a Virtual Hub Route Table.
///
/// > **Note:** Route table routes can managed with this resource, or in-line with the virtual_hub_route_table resource. Using both is not supported.
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
///     let exampleNetworkSecurityGroup = network_security_group::create(
///         "exampleNetworkSecurityGroup",
///         NetworkSecurityGroupArgs::builder()
///             .location("${example.location}")
///             .name("example-nsg")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleSubnet = subnet::create(
///         "exampleSubnet",
///         SubnetArgs::builder()
///             .address_prefixes(vec!["10.5.1.0/24",])
///             .name("examplesubnet")
///             .resource_group_name("${example.name}")
///             .virtual_network_name("${exampleVirtualNetwork.name}")
///             .build_struct(),
///     );
///     let exampleSubnetNetworkSecurityGroupAssociation = subnet_network_security_group_association::create(
///         "exampleSubnetNetworkSecurityGroupAssociation",
///         SubnetNetworkSecurityGroupAssociationArgs::builder()
///             .network_security_group_id("${exampleNetworkSecurityGroup.id}")
///             .subnet_id("${exampleSubnet.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.2.0/24")
///             .location("${example.location}")
///             .name("example-vhub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHubConnection = virtual_hub_connection::create(
///         "exampleVirtualHubConnection",
///         VirtualHubConnectionArgs::builder()
///             .name("example-vhubconn")
///             .remote_virtual_network_id("${exampleVirtualNetwork.id}")
///             .routing(
///                 VirtualHubConnectionRouting::builder()
///                     .associatedRouteTableId("${exampleVirtualHubRouteTable.id}")
///                     .build_struct(),
///             )
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHubRouteTable = virtual_hub_route_table::create(
///         "exampleVirtualHubRouteTable",
///         VirtualHubRouteTableArgs::builder()
///             .labels(vec!["label1",])
///             .name("example-vhubroutetable")
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHubRouteTableRoute = virtual_hub_route_table_route::create(
///         "exampleVirtualHubRouteTableRoute",
///         VirtualHubRouteTableRouteArgs::builder()
///             .destinations(vec!["10.0.0.0/16",])
///             .destinations_type("CIDR")
///             .name("example-route")
///             .next_hop("${exampleVirtualHubConnection.id}")
///             .next_hop_type("ResourceId")
///             .route_table_id("${exampleVirtualHubRouteTable.id}")
///             .build_struct(),
///     );
///     let exampleVirtualNetwork = virtual_network::create(
///         "exampleVirtualNetwork",
///         VirtualNetworkArgs::builder()
///             .address_spaces(vec!["10.5.0.0/16",])
///             .location("${example.location}")
///             .name("example-vnet")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
///     let exampleVirtualWan = virtual_wan::create(
///         "exampleVirtualWan",
///         VirtualWanArgs::builder()
///             .location("${example.location}")
///             .name("example-vwan")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Virtual Hub Route Table Routes can be imported using `<Route Table Resource Id>/routes/<Route Name>`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/virtualHubRouteTableRoute:VirtualHubRouteTableRoute example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/virtualHubs/virtualHub1/hubRouteTables/routeTable1/routes/routeName
/// ```
///
pub mod virtual_hub_route_table_route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubRouteTableRouteArgs {
        /// A list of destination addresses for this route.
        #[builder(into)]
        pub destinations: pulumi_wasm_rust::InputOrOutput<Vec<String>>,
        /// The type of destinations. Possible values are `CIDR`, `ResourceId` and `Service`.
        #[builder(into)]
        pub destinations_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name which should be used for this route. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The next hop's resource ID.
        #[builder(into)]
        pub next_hop: pulumi_wasm_rust::InputOrOutput<String>,
        /// The type of next hop. Currently the only possible value is `ResourceId`. Defaults to `ResourceId`.
        #[builder(into, default)]
        pub next_hop_type: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Hub Route Table to link this route to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_table_id: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubRouteTableRouteResult {
        /// A list of destination addresses for this route.
        pub destinations: pulumi_wasm_rust::Output<Vec<String>>,
        /// The type of destinations. Possible values are `CIDR`, `ResourceId` and `Service`.
        pub destinations_type: pulumi_wasm_rust::Output<String>,
        /// The name which should be used for this route. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The next hop's resource ID.
        pub next_hop: pulumi_wasm_rust::Output<String>,
        /// The type of next hop. Currently the only possible value is `ResourceId`. Defaults to `ResourceId`.
        pub next_hop_type: pulumi_wasm_rust::Output<Option<String>>,
        /// The ID of the Virtual Hub Route Table to link this route to. Changing this forces a new resource to be created.
        pub route_table_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: VirtualHubRouteTableRouteArgs,
    ) -> VirtualHubRouteTableRouteResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let destinations_binding = args.destinations.get_output(context).get_inner();
        let destinations_type_binding = args
            .destinations_type
            .get_output(context)
            .get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let next_hop_binding = args.next_hop.get_output(context).get_inner();
        let next_hop_type_binding = args.next_hop_type.get_output(context).get_inner();
        let route_table_id_binding = args.route_table_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/virtualHubRouteTableRoute:VirtualHubRouteTableRoute"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "destinations".into(),
                    value: &destinations_binding,
                },
                register_interface::ObjectField {
                    name: "destinationsType".into(),
                    value: &destinations_type_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nextHop".into(),
                    value: &next_hop_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopType".into(),
                    value: &next_hop_type_binding,
                },
                register_interface::ObjectField {
                    name: "routeTableId".into(),
                    value: &route_table_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        VirtualHubRouteTableRouteResult {
            destinations: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinations"),
            ),
            destinations_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("destinationsType"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            next_hop: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nextHop"),
            ),
            next_hop_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nextHopType"),
            ),
            route_table_id: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routeTableId"),
            ),
        }
    }
}
