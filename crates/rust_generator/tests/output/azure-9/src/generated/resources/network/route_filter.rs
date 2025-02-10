/// Manages a Route Filter.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = route_filter::create(
///         "example",
///         RouteFilterArgs::builder()
///             .location("East US")
///             .name("example")
///             .resource_group_name("example")
///             .rule(
///                 RouteFilterRule::builder()
///                     .access("Allow")
///                     .communities(vec!["12076:52004",])
///                     .name("rule")
///                     .ruleType("Community")
///                     .build_struct(),
///             )
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Route Filters can be imported using the `resource id`, e.g.
///
/// ```sh
/// $ pulumi import azure:network/routeFilter:RouteFilter example /subscriptions/00000000-0000-0000-0000-000000000000/resourceGroups/group1/providers/Microsoft.Network/routeFilters/routeFilter1
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod route_filter {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RouteFilterArgs {
        /// The Azure Region where the Route Filter should exist. Changing this forces a new Route Filter to be created.
        #[builder(into, default)]
        pub location: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The Name which should be used for this Route Filter.
        #[builder(into, default)]
        pub name: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the Resource Group where the Route Filter should exist. Changing this forces a new Route Filter to be created.
        #[builder(into)]
        pub resource_group_name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A `rule` block as defined below.
        #[builder(into, default)]
        pub rule: pulumi_gestalt_rust::InputOrOutput<
            Option<super::super::types::network::RouteFilterRule>,
        >,
        /// A mapping of tags which should be assigned to the Route Filter.
        #[builder(into, default)]
        pub tags: pulumi_gestalt_rust::InputOrOutput<
            Option<std::collections::HashMap<String, String>>,
        >,
    }
    #[allow(dead_code)]
    pub struct RouteFilterResult {
        /// The Azure Region where the Route Filter should exist. Changing this forces a new Route Filter to be created.
        pub location: pulumi_gestalt_rust::Output<String>,
        /// The Name which should be used for this Route Filter.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The name of the Resource Group where the Route Filter should exist. Changing this forces a new Route Filter to be created.
        pub resource_group_name: pulumi_gestalt_rust::Output<String>,
        /// A `rule` block as defined below.
        pub rule: pulumi_gestalt_rust::Output<
            super::super::types::network::RouteFilterRule,
        >,
        /// A mapping of tags which should be assigned to the Route Filter.
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
        args: RouteFilterArgs,
    ) -> RouteFilterResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let location_binding = args.location.get_output(context);
        let name_binding = args.name.get_output(context);
        let resource_group_name_binding = args.resource_group_name.get_output(context);
        let rule_binding = args.rule.get_output(context);
        let tags_binding = args.tags.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "azure:network/routeFilter:RouteFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: &[
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
                    name: "rule".into(),
                    value: rule_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "tags".into(),
                    value: tags_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RouteFilterResult {
            location: o.get_field("location"),
            name: o.get_field("name"),
            resource_group_name: o.get_field("resourceGroupName"),
            rule: o.get_field("rule"),
            tags: o.get_field("tags"),
        }
    }
}
