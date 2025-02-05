/// Manages a Route Table
///
/// > **NOTE on Route Tables and Routes:** There is both a standalone `route` resource, and allows for Routes to be defined in-line within the `route_table` resource.
/// At this time you cannot use a Route Table with in-line Routes in conjunction with any Route resources. Doing so will cause a conflict of Route configurations and will overwrite Routes.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: azure:core:ResourceGroup
///     properties:
///       name: example-resources
///       location: West Europe
///   exampleRouteTable:
///     type: azure:network:RouteTable
///     name: example
///     properties:
///       name: example-route-table
///       location: ${example.location}
///       resourceGroupName: ${example.name}
///       routes:
///         - name: route1
///           addressPrefix: 10.1.0.0/16
///           nextHopType: VnetLocal
///       tags:
///         environment: Production
/// ```
///
/// ## Import
///
/// Route Tables can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/routeTable:RouteTable example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/mygroup1/providers/Microsoft.Network/routeTables/mytable1
/// ```
///
pub mod route_table {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteTableArgs {
        /// Boolean flag which controls propagation of routes learned by BGP on that route table. Defaults to `true`.
        #[builder(into, default)]
        pub bgp_route_propagation_enabled: pulumi_wasm_rust::InputOrOutput<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the route.
        #[builder(into, default)]
        pub name: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the route table. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_wasm_rust::InputOrOutput<String>,
        /// A list of objects representing routes. Each object accepts the arguments documented below.
        ///
        /// > **NOTE** Since `route` can be configured both inline and via the separate `azure.network.Route` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub routes: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::super::types::network::RouteTableRoute>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_wasm_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RouteTableResult {
        /// Boolean flag which controls propagation of routes learned by BGP on that route table. Defaults to `true`.
        pub bgp_route_propagation_enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_wasm_rust::Output<String>,
        /// The name of the route.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The name of the resource group in which to create the route table. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_wasm_rust::Output<String>,
        /// A list of objects representing routes. Each object accepts the arguments documented below.
        ///
        /// > **NOTE** Since `route` can be configured both inline and via the separate `azure.network.Route` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub routes: pulumi_wasm_rust::Output<
            Vec<super::super::types::network::RouteTableRoute>,
        >,
        /// The collection of Subnets associated with this route table.
        pub subnets: pulumi_wasm_rust::Output<Vec<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_wasm_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RouteTableArgs,
    ) -> RouteTableResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let bgp_route_propagation_enabled_binding = args
            .bgp_route_propagation_enabled
            .get_output(context)
            .get_inner();
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let routes_binding = args.routes.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/routeTable:RouteTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "bgpRoutePropagationEnabled".into(),
                    value: &bgp_route_propagation_enabled_binding,
                },
                register_interface::ObjectField {
                    name: "location".into(),
                    value: &location_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "resourceGroupName".into(),
                    value: &resource_group_name_binding,
                },
                register_interface::ObjectField {
                    name: "routes".into(),
                    value: &routes_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouteTableResult {
            bgp_route_propagation_enabled: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("bgpRoutePropagationEnabled"),
            ),
            location: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_wasm_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            routes: pulumi_wasm_rust::__private::into_domain(o.extract_field("routes")),
            subnets: pulumi_wasm_rust::__private::into_domain(
                o.extract_field("subnets"),
            ),
            tags: pulumi_wasm_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
