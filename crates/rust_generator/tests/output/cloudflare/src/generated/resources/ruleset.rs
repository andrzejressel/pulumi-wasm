/// The [Cloudflare Ruleset Engine](https://developers.cloudflare.com/firewall/cf-rulesets)
/// allows you to create and deploy rules and rulesets.
///
/// The engine syntax, inspired by the Wireshark Display Filter language, is the
/// same syntax used in custom Firewall Rules. Cloudflare uses the Ruleset Engine
/// in different products, allowing you to configure several products using the same
/// basic syntax.
///
/// ## Import
///
/// Import an account scoped Ruleset configuration.
///
/// ```sh
/// $ pulumi import cloudflare:index/ruleset:Ruleset example account/<account_id>/<ruleset_id>
/// ```
///
/// Import a zone scoped Ruleset configuration.
///
/// ```sh
/// $ pulumi import cloudflare:index/ruleset:Ruleset example zone/<zone_id>/<ruleset_id>
/// ```
///
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod ruleset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RulesetArgs {
        /// The account identifier to target for the resource.
        #[builder(into, default)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Brief summary of the ruleset and its intended use.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
        #[builder(into)]
        pub kind: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Name of the ruleset.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
        #[builder(into)]
        pub phase: pulumi_gestalt_rust::InputOrOutput<String>,
        /// List of rules to apply to the ruleset.
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<super::types::RulesetRule>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into, default)]
        pub zone_id: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RulesetResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_gestalt_rust::Output<Option<String>>,
        /// Brief summary of the ruleset and its intended use.
        pub description: pulumi_gestalt_rust::Output<String>,
        /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
        pub kind: pulumi_gestalt_rust::Output<String>,
        /// Name of the ruleset.
        pub name: pulumi_gestalt_rust::Output<String>,
        /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
        pub phase: pulumi_gestalt_rust::Output<String>,
        /// List of rules to apply to the ruleset.
        pub rules: pulumi_gestalt_rust::Output<Option<Vec<super::types::RulesetRule>>>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_gestalt_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: RulesetArgs,
    ) -> RulesetResult {
        use pulumi_gestalt_rust::__private::pulumi_gestalt_wit::client_bindings::component::pulumi_gestalt::register_interface;
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let kind_binding = args.kind.get_output(context);
        let name_binding = args.name.get_output(context);
        let phase_binding = args.phase.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let zone_id_binding = args.zone_id.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/ruleset:Ruleset".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: &[
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "accountId".into(),
                    value: account_id_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "description".into(),
                    value: description_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "kind".into(),
                    value: kind_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "phase".into(),
                    value: phase_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "zoneId".into(),
                    value: zone_id_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        RulesetResult {
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            kind: o.get_field("kind"),
            name: o.get_field("name"),
            phase: o.get_field("phase"),
            rules: o.get_field("rules"),
            zone_id: o.get_field("zoneId"),
        }
    }
}
