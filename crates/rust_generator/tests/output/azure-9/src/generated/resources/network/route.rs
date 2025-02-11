/// Manages a Route within a Route Table.
///
/// > **NOTE on Route Tables and Routes:** This provider currently
/// provides both a standalone Route resource, and allows for Routes to be defined in-line within the Route Table resource.
/// At this time you cannot use a Route Table with in-line Routes in conjunction with any Route resources. Doing so will cause a conflict of Route configurations and will overwrite Routes.
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
///     let exampleRoute = route::create(
///         "exampleRoute",
///         RouteArgs::builder()
///             .address_prefix("10.1.0.0/16")
///             .name("acceptanceTestRoute1")
///             .next_hop_type("VnetLocal")
///             .resource_group_name("${example.name}")
///             .route_table_name("${exampleRouteTable.name}")
///             .build_struct(),
///     );
///     let exampleRouteTable = route_table::create(
///         "exampleRouteTable",
///         RouteTableArgs::builder()
///             .location("${example.location}")
///             .name("acceptanceTestRouteTable1")
///             .resource_group_name("${example.name}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Routes can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/route:Route exampleRoute /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/routeTables/mytable1/routes/myroute1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// The destination to which the route applies. Can be CIDR (such as `10.1.0.0/16`) or [Azure Service Tag](https://docs.microsoft.com/azure/virtual-network/service-tags-overview) (such as `ApiManagement`, `AzureBackup` or `AzureMonitor`) format.
        #[builder(into)]
        pub address_prefix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the route. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Contains the IP address packets should be forwarded to. Next hop values are only allowed in routes where the next hop type is `VirtualAppliance`.
        #[builder(into, default)]
        pub next_hop_in_ip_address: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The type of Azure hop the packet should be sent to. Possible values are `VirtualNetworkGateway`, `VnetLocal`, `Internet`, `VirtualAppliance` and `None`.
        #[builder(into)]
        pub next_hop_type: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the route. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The name of the route table within which create the route. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_table_name: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// The destination to which the route applies. Can be CIDR (such as `10.1.0.0/16`) or [Azure Service Tag](https://docs.microsoft.com/azure/virtual-network/service-tags-overview) (such as `ApiManagement`, `AzureBackup` or `AzureMonitor`) format.
        pub address_prefix: pulumi_gestalt_rust::Output<String>,
        /// The name of the route. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Contains the IP address packets should be forwarded to. Next hop values are only allowed in routes where the next hop type is `VirtualAppliance`.
        pub next_hop_in_ip_address: pulumi_gestalt_rust::Output<Option<String>>,
        /// The type of Azure hop the packet should be sent to. Possible values are `VirtualNetworkGateway`, `VnetLocal`, `Internet`, `VirtualAppliance` and `None`.
        pub next_hop_type: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the route. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// The name of the route table within which create the route. Changing this forces a new resource to be created.
        pub route_table_name: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteArgs,
    ) -> RouteResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let address_prefix_binding = args.address_prefix.get_output(context);
        let name_binding = args.name.get_output(context);
        let next_hop_in_ip_address_binding = args
            .next_hop_in_ip_address
            .get_output(context);
        let next_hop_type_binding = args.next_hop_type.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let route_table_name_binding = args.route_table_name.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/route:Route".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "addressPrefix".into(),
                    value: &address_prefix_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nextHopInIpAddress".into(),
                    value: &next_hop_in_ip_address_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nextHopType".into(),
                    value: &next_hop_type_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routeTableName".into(),
                    value: &route_table_name_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteResult {
            address_prefix: o.get_field("addressPrefix"),
            name: o.get_field("name"),
            next_hop_in_ip_address: o.get_field("nextHopInIpAddress"),
            next_hop_type: o.get_field("nextHopType"),
            resource_group_name: o.get_field("resourceGroupName"),
            route_table_name: o.get_field("routeTableName"),
        }
    }
}
