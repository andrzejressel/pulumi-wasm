/// Manages a Route within a Route Table.
///
/// > **NOTE on Route Tables and Routes:** This provider currently
/// provides both a standalone Route resource, and allows for Routes to be defined in-line within the Route Table resource.
/// At this time you cannot use a Route Table with in-line Routes in conjunction with any Route resources. Doing so will cause a conflict of Route configurations and will overwrite Routes.
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
pub mod route {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteArgs {
        /// The destination to which the route applies. Can be CIDR (such as `10.1.0.0/16`) or [Azure Service Tag](https://docs.microsoft.com/azure/virtual-network/service-tags-overview) (such as `ApiManagement`, `AzureBackup` or `AzureMonitor`) format.
        #[builder(into)]
        pub address_prefix: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the route. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Contains the IP address packets should be forwarded to. Next hop values are only allowed in routes where the next hop type is `VirtualAppliance`.
        #[builder(into, default)]
        pub next_hop_in_ip_address: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The type of Azure hop the packet should be sent to. Possible values are `VirtualNetworkGateway`, `VnetLocal`, `Internet`, `VirtualAppliance` and `None`.
        #[builder(into)]
        pub next_hop_type: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the resource group in which to create the route. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// The name of the route table within which create the route. Changing this forces a new resource to be created.
        #[builder(into)]
        pub route_table_name: pulumi_wasm_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteResult {
        /// The destination to which the route applies. Can be CIDR (such as `10.1.0.0/16`) or [Azure Service Tag](https://docs.microsoft.com/azure/virtual-network/service-tags-overview) (such as `ApiManagement`, `AzureBackup` or `AzureMonitor`) format.
        pub address_prefix: pulumi_wasm_rust::Output<String>,
        /// The name of the route. Changing this forces a new resource to be created.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Contains the IP address packets should be forwarded to. Next hop values are only allowed in routes where the next hop type is `VirtualAppliance`.
        pub next_hop_in_ip_address: pulumi_wasm_rust::Output<Option<String>>,
        /// The type of Azure hop the packet should be sent to. Possible values are `VirtualNetworkGateway`, `VnetLocal`, `Internet`, `VirtualAppliance` and `None`.
        pub next_hop_type: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the route. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// The name of the route table within which create the route. Changing this forces a new resource to be created.
        pub route_table_name: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RouteArgs,
    ) -> RouteResult {
        use pulumi_wasm_rust::__private::pulumi_gestalt_adapter_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let address_prefix_binding = args.address_prefix.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let next_hop_in_ip_address_binding = args
            .next_hop_in_ip_address
            .get_output(context)
            .get_inner();
        let next_hop_type_binding = args.next_hop_type.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let route_table_name_binding = args
            .route_table_name
            .get_output(context)
            .get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/route:Route".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "addressPrefix".into(),
                    value: &address_prefix_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopInIpAddress".into(),
                    value: &next_hop_in_ip_address_binding,
                },
                register_interface::ObjectField {
                    name: "nextHopType".into(),
                    value: &next_hop_type_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "routeTableName".into(),
                    value: &route_table_name_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouteResult {
            address_prefix: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("addressPrefix"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            next_hop_in_ip_address: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nextHopInIpAddress"),
            ),
            next_hop_type: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("nextHopType"),
            ),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            route_table_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("routeTableName"),
            ),
        }
    }
}
