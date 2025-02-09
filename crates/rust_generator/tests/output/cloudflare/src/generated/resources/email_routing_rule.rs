/// The [Email Routing Rule](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#email-rule-actions) resource allows you to create and manage email routing rules for a zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let main = email_routing_rule::create(
///         "main",
///         EmailRoutingRuleArgs::builder()
///             .actions(
///                 vec![
///                     EmailRoutingRuleAction::builder(). type ("forward")
///                     .values(vec!["destinationaddress@example.net",]).build_struct(),
///                 ],
///             )
///             .enabled(true)
///             .matchers(
///                 vec![
///                     EmailRoutingRuleMatcher::builder().field("to"). type ("literal")
///                     .value("test@example.com").build_struct(),
///                 ],
///             )
///             .name("terraform rule")
///             .zone_id("0da42c8d2132a9ddaf714f9e7c920711")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/emailRoutingRule:EmailRoutingRule example <zone_id>/<email_routing_rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod email_routing_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailRoutingRuleArgs {
        /// Actions to take when a match is found.
        #[builder(into, default)]
        pub actions: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::EmailRoutingRuleAction>>,
        >,
        /// Whether the email routing rule is enabled.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// Matching patterns to forward to your actions.
        #[builder(into, default)]
        pub matchers: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::EmailRoutingRuleMatcher>>,
        >,
        /// Routing rule name.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The priority of the email routing rule.
        #[builder(into, default)]
        pub priority: pulumi_gestalt_rust::InputOrOutput<Option<i32>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<String>,
    }
    #[allow(dead_code)]
    pub struct EmailRoutingRuleResult {
        /// Actions to take when a match is found.
        pub actions: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::EmailRoutingRuleAction>>,
        >,
        /// Whether the email routing rule is enabled.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// Matching patterns to forward to your actions.
        pub matchers: pulumi_gestalt_rust::Output<
            Option<Vec<super::types::EmailRoutingRuleMatcher>>,
        >,
        /// Routing rule name.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The priority of the email routing rule.
        pub priority: pulumi_gestalt_rust::Output<i32>,
        /// The tag of the email routing rule.
        pub tag: pulumi_gestalt_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::PulumiContext,
        name: &str,
        args: EmailRoutingRuleArgs,
    ) -> EmailRoutingRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        let actions_binding_1 = args.actions.get_output(context);
        let actions_binding = actions_binding_1.get_inner();
        let enabled_binding_1 = args.enabled.get_output(context);
        let enabled_binding = enabled_binding_1.get_inner();
        let matchers_binding_1 = args.matchers.get_output(context);
        let matchers_binding = matchers_binding_1.get_inner();
        let name_binding_1 = args.name.get_output(context);
        let name_binding = name_binding_1.get_inner();
        let priority_binding_1 = args.priority.get_output(context);
        let priority_binding = priority_binding_1.get_inner();
        let zone_id_binding_1 = args.zone_id.get_output(context);
        let zone_id_binding = zone_id_binding_1.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingRule:EmailRoutingRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "actions".into(),
                    value: &actions_binding,
                },
                register_interface::ObjectField {
                    name: "enabled".into(),
                    value: &enabled_binding,
                },
                register_interface::ObjectField {
                    name: "matchers".into(),
                    value: &matchers_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "priority".into(),
                    value: &priority_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        EmailRoutingRuleResult {
            actions: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("actions"),
            ),
            enabled: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("enabled"),
            ),
            matchers: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("matchers"),
            ),
            name: pulumi_gestalt_rust::__private::into_domain(o.extract_field("name")),
            priority: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("priority"),
            ),
            tag: pulumi_gestalt_rust::__private::into_domain(o.extract_field("tag")),
            zone_id: pulumi_gestalt_rust::__private::into_domain(
                o.extract_field("zoneId"),
            ),
        }
    }
}
