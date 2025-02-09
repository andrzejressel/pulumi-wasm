/// Provides a Cloudflare Teams rule resource. Teams rules comprise secure web gateway policies.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_gestalt_rust::Output;
/// use pulumi_gestalt_rust::{add_export, pulumi_main};
/// #[pulumi_main]
/// fn test_main() -> Result<(), Error> {
///     let example = teams_rule::create(
///         "example",
///         TeamsRuleArgs::builder()
///             .account_id("f037e56e89293a057740de681ac9abbe")
///             .action("block")
///             .description("desc")
///             .filters(vec!["http",])
///             .name("office")
///             .precedence(1)
///             .rule_settings(
///                 TeamsRuleRuleSettings::builder()
///                     .blockPageEnabled(true)
///                     .blockPageReason("access not permitted")
///                     .build_struct(),
///             )
///             .traffic("http.request.uri == \"https://www.example.com/malicious\"")
///             .build_struct(),
///     );
/// }
/// ```
///
/// ## Import
///
/// ```sh
/// $ pulumi import cloudflare:index/teamsRule:TeamsRule example <account_id>/<teams_rule_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod teams_rule {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct TeamsRuleArgs {
        /// The account identifier to target for the resource.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
        #[builder(into)]
        pub action: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The description of the teams rule.
        #[builder(into)]
        pub description: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The wirefilter expression to be used for device_posture check matching.
        #[builder(into, default)]
        pub device_posture: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Indicator of rule enablement.
        #[builder(into, default)]
        pub enabled: pulumi_gestalt_rust::InputOrOutput<Option<bool>>,
        /// The protocol or layer to evaluate the traffic and identity expressions.
        #[builder(into, default)]
        pub filters: pulumi_gestalt_rust::InputOrOutput<Option<Vec<String>>>,
        /// The wirefilter expression to be used for identity matching.
        #[builder(into, default)]
        pub identity: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the teams rule.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// The evaluation precedence of the teams rule.
        #[builder(into)]
        pub precedence: pulumi_gestalt_rust::InputOrOutput<i32>,
        /// Additional rule settings.
        #[builder(into, default)]
        pub rule_settings: pulumi_gestalt_rust::InputOrOutput<
            Option<super::types::TeamsRuleRuleSettings>,
        >,
        /// The wirefilter expression to be used for traffic matching.
        #[builder(into, default)]
        pub traffic: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct TeamsRuleResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// The action executed by matched teams rule. Available values: `allow`, `block`, `safesearch`, `ytrestricted`, `on`, `off`, `scan`, `noscan`, `isolate`, `noisolate`, `override`, `l4_override`, `egress`, `audit_ssh`, `resolve`.
        pub action: pulumi_gestalt_rust::Output<String>,
        /// The description of the teams rule.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// The wirefilter expression to be used for device_posture check matching.
        pub device_posture: pulumi_gestalt_rust::Output<String>,
        /// Indicator of rule enablement.
        pub enabled: pulumi_gestalt_rust::Output<Option<bool>>,
        /// The protocol or layer to evaluate the traffic and identity expressions.
        pub filters: pulumi_gestalt_rust::Output<Option<Vec<String>>>,
        /// The wirefilter expression to be used for identity matching.
        pub identity: pulumi_gestalt_rust::Output<String>,
        /// The name of the teams rule.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// The evaluation precedence of the teams rule.
        pub precedence: pulumi_gestalt_rust::Output<i32>,
        /// Additional rule settings.
        pub rule_settings: pulumi_gestalt_rust::Output<
            super::types::TeamsRuleRuleSettings,
        >,
        /// The wirefilter expression to be used for traffic matching.
        pub traffic: pulumi_gestalt_rust::Output<String>,
        pub version: pulumi_gestalt_rust::Output<i32>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: TeamsRuleArgs,
    ) -> TeamsRuleResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let action_binding = args.action.get_output(context);
        let description_binding = args.description.get_output(context);
        let device_posture_binding = args.device_posture.get_output(context);
        let enabled_binding = args.enabled.get_output(context);
        let filters_binding = args.filters.get_output(context);
        let identity_binding = args.identity.get_output(context);
        let name_binding = args.name.get_output(context);
        let precedence_binding = args.precedence.get_output(context);
        let rule_settings_binding = args.rule_settings.get_output(context);
        let traffic_binding = args.traffic.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/teamsRule:TeamsRule".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "action".into(),
                    value: action_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "devicePosture".into(),
                    value: device_posture_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "enabled".into(),
                    value: enabled_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "filters".into(),
                    value: filters_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "identity".into(),
                    value: identity_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "precedence".into(),
                    value: precedence_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "ruleSettings".into(),
                    value: rule_settings_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "traffic".into(),
                    value: traffic_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        TeamsRuleResult {
            account_id: o.get_field("accountId"),
            action: o.get_field("action"),
            description: o.get_field("description"),
            device_posture: o.get_field("devicePosture"),
            enabled: o.get_field("enabled"),
            filters: o.get_field("filters"),
            identity: o.get_field("identity"),
            name: o.get_field("name"),
            precedence: o.get_field("precedence"),
            rule_settings: o.get_field("ruleSettings"),
            traffic: o.get_field("traffic"),
            version: o.get_field("version"),
        }
    }
}
