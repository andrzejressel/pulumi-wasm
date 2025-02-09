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
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: UserAgentBlockingRuleArgs,
    ) -> UserAgentBlockingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let configuration_binding_1 = args.configuration.get_output(context);
        let configuration_binding = configuration_binding_1.get_inner();
        let description_binding_1 = args.description.get_output(context);
        let description_binding = description_binding_1.get_inner();
        let mode_binding_1 = args.mode.get_output(context);
        let mode_binding = mode_binding_1.get_inner();
        let paused_binding_1 = args.paused.get_output(context);
        let paused_binding = paused_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "configuration".into(),
                    value: &configuration_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "mode".into(),
                    value: &mode_binding,
                },
                register_interface::ObjectField {
                    name: "paused".into(),
                    value: &paused_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        UserAgentBlockingRuleResult {
            configuration: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("configuration"),
            ),
            description: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("description"),
            ),
            mode: pulumi_gestalt_rust::__private::into_domain(o.extract_field("mode")),
            paused: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("paused"),
            ),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
