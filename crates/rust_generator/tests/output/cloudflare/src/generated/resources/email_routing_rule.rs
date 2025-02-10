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
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: EmailRoutingRuleArgs,
    ) -> EmailRoutingRuleResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let actions_binding = args.actions.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let matchers_binding = args.matchers.get_output(context);
        let name_binding = args.name.get_output(context);
        let priority_binding = args.priority.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/emailRoutingRule:EmailRoutingRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "actions".into(),
                    value: actions_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "matchers".into(),
                    value: matchers_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "priority".into(),
                    value: priority_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        EmailRoutingRuleResult {
            actions: o.get_field("actions"),
            enabled: o.get_field("enabled"),
            matchers: o.get_field("matchers"),
            name: o.get_field("name"),
            priority: o.get_field("priority"),
            tag: o.get_field("tag"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
