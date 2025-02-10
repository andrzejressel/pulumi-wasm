/// Manages a Route Map.
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
///     let exampleRouteMap = route_map::create(
///         "exampleRouteMap",
///         RouteMapArgs::builder()
///             .name("example-rm")
///             .rules(
///                 vec![
///                     RouteMapRule::builder().actions(vec![RouteMapRuleAction::builder()
///                     .parameters(vec![RouteMapRuleActionParameter::builder()
///                     .asPaths(vec!["22334",]).build_struct(),]). type ("Add")
///                     .build_struct(),])
///                     .matchCriterions(vec![RouteMapRuleMatchCriterion::builder()
///                     .matchCondition("Contains").routePrefixes(vec!["10.0.0.0/8",])
///                     .build_struct(),]).name("rule1").nextStepIfMatched("Continue")
///                     .build_struct(),
///                 ],
///             )
///             .virtual_hub_id("${exampleVirtualHub.id}")
///             .build_struct(),
///     );
///     let exampleVirtualHub = virtual_hub::create(
///         "exampleVirtualHub",
///         VirtualHubArgs::builder()
///             .address_prefix("10.0.1.0/24")
///             .location("${example.location}")
///             .name("example-vhub")
///             .resource_group_name("${example.name}")
///             .virtual_wan_id("${exampleVirtualWan.id}")
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
/// Route Maps can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/routeMap:RouteMap example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/resourceGroup1/providers/Microsoft.Network/virtualHubs/virtualHub1/routeMaps/routeMap1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_map {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteMapArgs {
        /// The name which should be used for this Route Map. Changing this forces a new resource to be created.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// A `rule` block as defined below.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::super::types::network::RouteMapRule>>,
        >,
        /// The resource ID of the Virtual Hub. Changing this forces a new resource to be created.
        #[builder(into)]
        pub virtual_hub_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct RouteMapResult {
        /// The name which should be used for this Route Map. Changing this forces a new resource to be created.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// A `rule` block as defined below.
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<super::super::types::network::RouteMapRule>>,
        >,
        /// The resource ID of the Virtual Hub. Changing this forces a new resource to be created.
        pub virtual_hub_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RouteMapArgs,
    ) -> RouteMapResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let name_binding = args.name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let virtual_hub_id_binding = args.virtual_hub_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/routeMap:RouteMap".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "virtualHubId".into(),
                    value: virtual_hub_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteMapResult {
            name: o.get_field("name"),
            rules: o.get_field("rules"),
            virtual_hub_id: o.get_field("virtualHubId"),
        }
    }
}
