/// Provides a resource to manage User Agent Blocking Rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example1 = user_agent_blocking_rule::create(
///         "example1",
///         UserAgentBlockingRuleArgs::builder()
///             .configuration(
///                 UserAgentBlockingRuleConfiguration::builder()
///                     .target("ua")
///                     .value("Chrome")
///                     .build_struct(),
///             )
///             .description("My description 1")
///             .mode("js_challenge")
///             .paused(false)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
///     let example2 = user_agent_blocking_rule::create(
///         "example2",
///         UserAgentBlockingRuleArgs::builder()
///             .configuration(
///                 UserAgentBlockingRuleConfiguration::builder()
///                     .target("ua")
///                     .value("Mozilla")
///                     .build_struct(),
///             )
///             .description("My description 22")
///             .mode("challenge")
///             .paused(true)
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule example <zone_id>/<user_agent_blocking_rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod user_agent_blocking_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserAgentBlockingRuleArgs {
        /// The configuration object for the current rule.
        #[builder(into)]
        pub configuration: pulumi_gestalt_rust::InputOrOutput<
            super::types::UserAgentBlockingRuleConfiguration,
        >,
        /// An informative summary of the rule.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
        #[builder(into)]
        pub mode: pulumi_gestalt_rust::InputOrOutput<String>,
        /// When true, indicates that the rule is currently paused.
        #[builder(into)]
        pub paused: pulumi_gestalt_rust::InputOrOutput<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct UserAgentBlockingRuleResult {
        /// The configuration object for the current rule.
        pub configuration: pulumi_gestalt_rust::Output<
            super::types::UserAgentBlockingRuleConfiguration,
        >,
        /// An informative summary of the rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
        pub mode: pulumi_gestalt_rust::Output<String>,
        /// When true, indicates that the rule is currently paused.
        pub paused: pulumi_gestalt_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: UserAgentBlockingRuleArgs,
    ) -> UserAgentBlockingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let configuration_binding = args.configuration.get_output(context);
        let description_binding = args.description.get_output(context);
        let mode_binding = args.mode.get_output(context);
        let paused_binding = args.paused.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "configuration".into(),
                    value: configuration_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "mode".into(),
                    value: mode_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "paused".into(),
                    value: paused_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        UserAgentBlockingRuleResult {
            configuration: o.get_field("configuration"),
            description: o.get_field("description"),
            mode: o.get_field("mode"),
            paused: o.get_field("paused"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
