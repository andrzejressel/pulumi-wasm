/// Provides a resource, that manages Cloudflare static routes for Magic
/// Transit or Magic WAN. Static routes are used to route traffic
/// through GRE tunnels.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = magic_wan_static_route::create(
///         "example",
///         MagicWanStaticRouteArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .colo_names(vec!["den01",])
///             .colo_regions(vec!["APAC",])
///             .description("New route for new prefix 192.0.2.0/24")
///             .nexthop("10.0.0.0")
///             .prefix("192.0.2.0/24")
///             .priority(100)
///             .weight(10)
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/magicWanStaticRoute:MagicWanStaticRoute example <account_id>/<static_route_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod magic_wan_static_route {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MagicWanStaticRouteArgs {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// List of Cloudflare colocation regions for this static route.
        #[builder(into, default)]
        pub colo_names: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// List of Cloudflare colocation names for this static route.
        #[builder(into, default)]
        pub colo_regions: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// Description of the static route.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The nexthop IP address where traffic will be routed to.
        #[builder(into)]
        pub nexthop: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Your network prefix using CIDR notation.
        #[builder(into)]
        pub prefix: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The priority for the static route.
        #[builder(into)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
        #[builder(into, default)]
        pub weight: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
    }
    #[allow(dead_code)]
    pub struct MagicWanStaticRouteResult {
        /// The account identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// List of Cloudflare colocation regions for this static route.
        pub colo_names: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// List of Cloudflare colocation names for this static route.
        pub colo_regions: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// Description of the static route.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The nexthop IP address where traffic will be routed to.
        pub nexthop: pulumi_gestalt_rust::Output<String>,
        /// Your network prefix using CIDR notation.
        pub prefix: pulumi_gestalt_rust::Output<String>,
        /// The priority for the static route.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The optional weight for ECMP routes. **Modifying this attribute will force creation of a new resource.**
        pub weight: pulumi_gestalt_rust::Output<Option<i32>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MagicWanStaticRouteArgs,
    ) -> MagicWanStaticRouteResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let colo_names_binding = args.colo_names.get_output(context);
        let colo_regions_binding = args.colo_regions.get_output(context);
        let description_binding = args.description.get_output(context);
        let nexthop_binding = args.nexthop.get_output(context);
        let prefix_binding = args.prefix.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let weight_binding = args.weight.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/magicWanStaticRoute:MagicWanStaticRoute".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coloNames".into(),
                    value: colo_names_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "coloRegions".into(),
                    value: colo_regions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "nexthop".into(),
                    value: nexthop_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "prefix".into(),
                    value: prefix_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "weight".into(),
                    value: weight_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MagicWanStaticRouteResult {
            account_id: o.get_field("accountId"),
            colo_names: o.get_field("coloNames"),
            colo_regions: o.get_field("coloRegions"),
            description: o.get_field("description"),
            nexthop: o.get_field("nexthop"),
            prefix: o.get_field("prefix"),
            priority: o.get_field("priority"),
            weight: o.get_field("weight"),
        }
    }
}
