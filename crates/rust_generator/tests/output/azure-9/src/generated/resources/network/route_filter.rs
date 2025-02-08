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
#[allow(clippy::doc_lazy_continuation)]
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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: RouteFilterArgs,
    ) -> RouteFilterResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let location_binding = args.location.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let resource_group_name_binding = args
            .resource_group_name
            .get_output(context)
            .get_inner();
        let rule_binding = args.rule.get_output(context).get_inner();
        let tags_binding = args.tags.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "azure:network/routeFilter:RouteFilter".into(),
            name: name.to_string(),
            version: super::super::get_version(),
            object: Vec::from([
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
                    name: "rule".into(),
                    value: &rule_binding,
                },
                register_interface::ObjectField {
                    name: "tags".into(),
                    value: &tags_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        RouteFilterResult {
            location: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("location"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            resource_group_name: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("resourceGroupName"),
            ),
            rule: pulumi_gestalt_rust::__private::into_domain(o.extract_field("rule")),
            tags: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tags")),
        }
    }
}
