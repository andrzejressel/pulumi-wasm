/// Provides a resource to manage User Agent Blocking Rules.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod user_agent_blocking_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct UserAgentBlockingRuleArgs {
        /// The configuration object for the current rule.
        #[builder(into)]
        pub configuration: pulumi_wasm_rust::Output<
            super::types::UserAgentBlockingRuleConfiguration,
        >,
        /// An informative summary of the rule.
        #[builder(into)]
        pub description: pulumi_wasm_rust::Output<String>,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
        #[builder(into)]
        pub mode: pulumi_wasm_rust::Output<String>,
        /// When true, indicates that the rule is currently paused.
        #[builder(into)]
        pub paused: pulumi_wasm_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct UserAgentBlockingRuleResult {
        /// The configuration object for the current rule.
        pub configuration: pulumi_wasm_rust::Output<
            super::types::UserAgentBlockingRuleConfiguration,
        >,
        /// An informative summary of the rule.
        pub description: pulumi_wasm_rust::Output<String>,
        /// The action to apply to a matched request. Available values: `block`, `challenge`, `js_challenge`, `managed_challenge`.
        pub mode: pulumi_wasm_rust::Output<String>,
        /// When true, indicates that the rule is currently paused.
        pub paused: pulumi_wasm_rust::Output<bool>,
        /// The zone identifier to target for the resource. **Modifying this attribute will force creation of a new resource.**
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        name: &str,
        args: UserAgentBlockingRuleArgs,
    ) -> UserAgentBlockingRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let configuration_binding = args.configuration.get_inner();
        let description_binding = args.description.get_inner();
        let mode_binding = args.mode.get_inner();
        let paused_binding = args.paused.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/userAgentBlockingRule:UserAgentBlockingRule".into(),
            name: name.to_string(),
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "configuration".into(),
                },
                register_interface::ResultField {
                    name: "description".into(),
                },
                register_interface::ResultField {
                    name: "mode".into(),
                },
                register_interface::ResultField {
                    name: "paused".into(),
                },
                register_interface::ResultField {
                    name: "zoneId".into(),
                },
            ]),
        };
        let o = register_interface::register(&request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        UserAgentBlockingRuleResult {
            configuration: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("configuration").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            mode: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("mode").unwrap(),
            ),
            paused: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("paused").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
