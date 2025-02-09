/// Provides a Cloudflare page rule resource.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let foobar = page_rule::create(
///         "foobar",
///         PageRuleArgs::builder()
///             .actions(
///                 PageRuleActions::builder()
///                     .emailObfuscation("on")
///                     .minifies(
///                         vec![
///                             PageRuleActionsMinify::builder().css("on").html("off")
///                             .js("on").build_struct(),
///                         ],
///                     )
///                     .ssl("flexible")
///                     .build_struct(),
///             )
///             .priority(1)
///             .target("sub.${cloudflareZone}/page")
///             .zone_id("${cloudflareZoneId}")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// Page rules can be imported using a composite ID formed of zone ID and page rule ID, e.g.
///
/// ```sh
/// $ pulumi import cloudflare:index/pageRule:PageRule default d41d8cd98f00b204e9800998ecf8427e/ch8374ftwdghsif43
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod page_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct PageRuleArgs {
        /// The actions taken by the page rule, options given below.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<super::types::PageRuleActions>,
        /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// Whether the page rule is active or disabled.
        #[builder(into, default)]
        pub status: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The URL pattern to target with the page rule.
        #[builder(into)]
        pub target: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The DNS zone ID to which the page rule should be added.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct PageRuleResult {
        /// The actions taken by the page rule, options given below.
        pub actions: pulumi_gestalt_rust::Output<super::types::PageRuleActions>,
        /// The priority of the page rule among others for this target, the higher the number the higher the priority as per [API documentation](https://api.cloudflare.com/#page-rules-for-a-zone-create-page-rule).
        pub priority: pulumi_gestalt_rust::Output<Option<i32>>,
        /// Whether the page rule is active or disabled.
        pub status: pulumi_gestalt_rust::Output<Option<String>>,
        /// The URL pattern to target with the page rule.
        pub target: pulumi_gestalt_rust::Output<String>,
        /// The DNS zone ID to which the page rule should be added.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: PageRuleArgs,
    ) -> PageRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let actions_binding_1 = args.actions.get_output(context);
        let actions_binding = actions_binding_1.get_inner();
        let priority_binding_1 = args.priority.get_output(context);
        let priority_binding = priority_binding_1.get_inner();
        let status_binding_1 = args.status.get_output(context);
        let status_binding = status_binding_1.get_inner();
        let target_binding_1 = args.target.get_output(context);
        let target_binding = target_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/pageRule:PageRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "status".into(),
                    value: &status_binding,
                },
                register_interface::ObjectField {
                    name: "target".into(),
                    value: &target_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        PageRuleResult {
            actions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            status: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("status"),
            ),
            target: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("target"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
