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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_table {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteTableArgs {
        /// Boolean flag which controls propagation of routes learned by BGP on that route table. Defaults to `true`.
        #[builder(into, default)]
        pub bgp_route_propagation_enabled: pulumi_gestalt_rust::InputOrOutput<
            Option<bool>,
        >,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the route.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the resource group in which to create the route table. Changing this forces a new resource to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A list of objects representing routes. Each object accepts the arguments documented below.
        ///
        /// > **NOTE** Since `route` can be configured both inline and via the separate `azure.network.Route` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        #[builder(into, default)]
        pub routes: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::RouteTableRoute>>,
        >,
        /// A mapping of tags to assign to the resource.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RouteTableResult {
        /// Boolean flag which controls propagation of routes learned by BGP on that route table. Defaults to `true`.
        pub bgp_route_propagation_enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Specifies the supported Azure location where the resource exists. Changing this forces a new resource to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The name of the route.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the resource group in which to create the route table. Changing this forces a new resource to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A list of objects representing routes. Each object accepts the arguments documented below.
        ///
        /// > **NOTE** Since `route` can be configured both inline and via the separate `azure.network.Route` resource, we have to explicitly set it to empty slice (`[]`) to remove it.
        pub routes: pulumi_gestalt_rust::Output<
            Vec<super::super::types::network::RouteTableRoute>,
        >,
        /// The collection of Subnets associated with this route table.
        pub subnets: pulumi_gestalt_rust::Output<Vec<String>>,
        /// A mapping of tags to assign to the resource.
        pub tags: pulumi_gestalt_rust::Output<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteTableArgs,
    ) -> RouteTableResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let bgp_route_propagation_enabled_binding = args
            .bgp_route_propagation_enabled
            .get_output(context);
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let routes_binding = args.routes.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/routeTable:RouteTable".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "bgpRoutePropagationEnabled".into(),
                    value: bgp_route_propagation_enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "location".into(),
                    value: location_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "resourceGroupName".into(),
                    value: resource_group_name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "routes".into(),
                    value: routes_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteTableResult {
            bgp_route_propagation_enabled: o.get_field("bgpRoutePropagationEnabled"),
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            routes: o.get_field("routes"),
            subnets: o.get_field("subnets"),
            tags: o.get_field("tags"),
        }
    }
}
