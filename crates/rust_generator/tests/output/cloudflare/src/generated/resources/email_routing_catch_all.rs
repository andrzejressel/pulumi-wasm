/// Provides a resource for managing Email Routing Addresses catch all behaviour.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = email_routing_catch_all::create(
///         "example",
///         EmailRoutingCatchAllArgs::builder()
///             .actions(
///                 vec![
///                     EmailRoutingCatchAllAction::builder(). type ("forward")
///                     .values(vec!["destinationaddress@example.net",]).build_struct(),
///                 ],
///             )
///             .enabled(true)
///             .matchers(
///                 vec![
///                     EmailRoutingCatchAllMatcher::builder(). type ("all").build_struct(),
///                 ],
///             )
///             .name("example catch all")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_routing_catch_all {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailRoutingCatchAllArgs {
        /// List actions patterns.
        #[builder(into)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::EmailRoutingCatchAllAction>,
        >,
        /// Routing rule status.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Matching patterns to forward to your actions.
        #[builder(into)]
        pub matchers: pulumi_gestalt_rust::InputOrOutput<
            Vec<super::types::EmailRoutingCatchAllMatcher>,
        >,
        /// Routing rule name.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailRoutingCatchAllResult {
        /// List actions patterns.
        pub actions: pulumi_gestalt_rust::Output<
            Vec<super::types::EmailRoutingCatchAllAction>,
        >,
        /// Routing rule status.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Matching patterns to forward to your actions.
        pub matchers: pulumi_gestalt_rust::Output<
            Vec<super::types::EmailRoutingCatchAllMatcher>,
        >,
        /// Routing rule name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Routing rule identifier.
        pub tag: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailRoutingCatchAllArgs,
    ) -> EmailRoutingCatchAllResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let matchers_binding = args.matchers.get_output(context);
        let name_binding = args.name.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingCatchAll:EmailRoutingCatchAll".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: &actions_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matchers".into(),
                    value: &matchers_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: &name_binding.drop_type(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding.drop_type(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailRoutingCatchAllResult {
            actions: o.get_field("actions"),
            enabled: o.get_field("enabled"),
            matchers: o.get_field("matchers"),
            name: o.get_field("name"),
            tag: o.get_field("tag"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
