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
pub mod ruleset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct RulesetArgs {
        /// The account identifier to target for the resource.
        #[builder(into, default)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Brief summary of the ruleset and its intended use.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
        #[builder(into)]
        pub kind: pulumi_wasm_rust::InputOrOutput<String>,
        /// Name of the ruleset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
        #[builder(into)]
        pub phase: pulumi_wasm_rust::InputOrOutput<String>,
        /// List of rules to apply to the ruleset.
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<super::types::RulesetRule>>,
        >,
        /// The zone identifier to target for the resource.
        #[builder(into, default)]
        pub zone_id: pulumi_wasm_rust::InputOrOutput<Option<String>>,
    }
    #[allow(dead_code)]
    pub struct RulesetResult {
        /// The account identifier to target for the resource.
        pub account_id: pulumi_wasm_rust::Output<Option<String>>,
        /// Brief summary of the ruleset and its intended use.
        pub description: pulumi_wasm_rust::Output<String>,
        /// Type of Ruleset to create. Available values: `custom`, `managed`, `root`, `zone`.
        pub kind: pulumi_wasm_rust::Output<String>,
        /// Name of the ruleset.
        pub name: pulumi_wasm_rust::Output<String>,
        /// Point in the request/response lifecycle where the ruleset will be created. Available values: `ddos_l4`, `ddos_l7`, `http_config_settings`, `http_custom_errors`, `http_log_custom_fields`, `http_ratelimit`, `http_request_cache_settings`, `http_request_dynamic_redirect`, `http_request_firewall_custom`, `http_request_firewall_managed`, `http_request_late_transform`, `http_request_origin`, `http_request_redirect`, `http_request_sanitize`, `http_request_sbfm`, `http_request_transform`, `http_response_compression`, `http_response_firewall_managed`, `http_response_headers_transform`, `magic_transit`.
        pub phase: pulumi_wasm_rust::Output<String>,
        /// List of rules to apply to the ruleset.
        pub rules: pulumi_wasm_rust::Output<Option<Vec<super::types::RulesetRule>>>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: RulesetArgs,
    ) -> RulesetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let kind_binding = args.kind.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let phase_binding = args.phase.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let zone_id_binding = args.zone_id.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/ruleset:Ruleset".into(),
            name: name.to_string(),
            version: super::get_version(),
            object: Vec::from([
                register_interface::ObjectField {
                    name: "accountId".into(),
                    value: &account_id_binding,
                },
                register_interface::ObjectField {
                    name: "description".into(),
                    value: &description_binding,
                },
                register_interface::ObjectField {
                    name: "kind".into(),
                    value: &kind_binding,
                },
                register_interface::ObjectField {
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "phase".into(),
                    value: &phase_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
                },
                register_interface::ObjectField {
                    name: "zoneId".into(),
                    value: &zone_id_binding,
                },
            ]),
            results: Vec::from([
                register_interface::ResultField {
                    name: "accountId".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "kind".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "phase".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        RulesetResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            kind: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("kind").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            phase: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("phase").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
