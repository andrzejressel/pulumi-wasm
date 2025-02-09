/// Manages a Route in a Virtual Hub Route Table.
///
/// > **Note:** Route table routes can managed with this resource, or in-line with the virtual_hub_route_table resource. Using both is not supported.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod virtual_hub_route_table_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct VirtualHubRouteTableRouteArgs {
        /// A list of destination addresses for this route.
        #[builder(into)]
        pub destinations: pulumi_gestalt_rust::InputOrOutput<Vec<String>>,
        /// The type of destinations. Possible values are `CIDR`, `ResourceId` and `Service`.
        #[builder(into)]
        pub destinations_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name which should be used for this route. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The next hop's resource ID.
        #[builder(into)]
        pub next_hop: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The type of next hop. Currently the only possible value is `ResourceId`. Defaults to `ResourceId`.
        #[builder(into, default)]
        pub next_hop_type: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The ID of the Virtual Hub Route Table to link this route to. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_table_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct VirtualHubRouteTableRouteResult {
        /// A list of destination addresses for this route.
        pub destinations: pulumi_gestalt_rust::Output<Vec<String>>,
        /// The type of destinations. Possible values are `CIDR`, `ResourceId` and `Service`.
        pub destinations_type: pulumi_gestalt_rust::Output<String>,
        /// The name which should be used for this route. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The next hop's resource ID.
        pub next_hop: pulumi_gestalt_rust::Output<String>,
        /// The type of next hop. Currently the only possible value is `ResourceId`. Defaults to `ResourceId`.
        pub next_hop_type: pulumi_gestalt_rust::Output<Option<String>>,
        /// The ID of the Virtual Hub Route Table to link this route to. Changing this forces a new resource to be created.
        pub route_table_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: VirtualHubRouteTableRouteArgs,
    ) -> VirtualHubRouteTableRouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let destinations_binding = args.destinations.get_output(context);
        let destinations_type_binding = args.destinations_type.get_output(context);
        let name_binding = args.name.get_output(context);
        let next_hop_binding = args.next_hop.get_output(context);
        let next_hop_type_binding = args.next_hop_type.get_output(context);
        let route_table_id_binding = args.route_table_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/virtualHubRouteTableRoute:VirtualHubRouteTableRoute"
                .into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinations".into(),
                    value: destinations_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "destinationsType".into(),
                    value: destinations_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nextHop".into(),
                    value: next_hop_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nextHopType".into(),
                    value: next_hop_type_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableId".into(),
                    value: route_table_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        VirtualHubRouteTableRouteResult {
            destinations: o.get_field("destinations"),
            destinations_type: o.get_field("destinationsType"),
            name: o.get_field("name"),
            next_hop: o.get_field("nextHop"),
            next_hop_type: o.get_field("nextHopType"),
            route_table_id: o.get_field("routeTableId"),
        }
    }
}
