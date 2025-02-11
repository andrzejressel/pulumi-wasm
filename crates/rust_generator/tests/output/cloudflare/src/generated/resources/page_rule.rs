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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: PageRuleArgs,
    ) -> PageRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let status_binding = args.status.get_output(context);
        let target_binding = args.target.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/pageRule:PageRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: &priority_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "status".into(),
                    value: &status_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "target".into(),
                    value: &target_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        PageRuleResult {
            actions: o.get_field("actions"),
            priority: o.get_field("priority"),
            status: o.get_field("status"),
            target: o.get_field("target"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
