/// The [Email Routing Rule](https://developers.cloudflare.com/email-routing/setup/email-routing-addresses/#email-rule-actions) resource allows you to create and manage email routing rules for a zone.
///
/// ## Example Usage
///
/// ```ignore
/// use pulumi_wasm_rust::Output;
/// use pulumi_wasm_rust::{add_export, pulumi_main};
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
pub mod email_routing_rule {
    #[derive(pulumi_wasm_rust::__private::bon::Builder, Clone)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct EmailRoutingRuleArgs {
        /// Actions to take when a match is found.
        #[builder(into, default)]
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::types::EmailRoutingRuleAction>>,
        >,
        /// Whether the email routing rule is enabled.
        #[builder(into, default)]
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Matching patterns to forward to your actions.
        #[builder(into, default)]
        pub matchers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::EmailRoutingRuleMatcher>>,
        >,
        /// Routing rule name.
        #[builder(into)]
        pub name: pulumi_wasm_rust::Output<String>,
        /// The priority of the email routing rule.
        #[builder(into, default)]
        pub priority: pulumi_wasm_rust::Output<Option<i32>>,
        /// The zone identifier to target for the resource.
        #[builder(into)]
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    #[allow(dead_code)]
    pub struct EmailRoutingRuleResult {
        /// Actions to take when a match is found.
        pub actions: pulumi_wasm_rust::Output<
            Option<Vec<super::types::EmailRoutingRuleAction>>,
        >,
        /// Whether the email routing rule is enabled.
        pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
        /// Matching patterns to forward to your actions.
        pub matchers: pulumi_wasm_rust::Output<
            Option<Vec<super::types::EmailRoutingRuleMatcher>>,
        >,
        /// Routing rule name.
        pub name: pulumi_wasm_rust::Output<String>,
        /// The priority of the email routing rule.
        pub priority: pulumi_wasm_rust::Output<i32>,
        /// The tag of the email routing rule.
        pub tag: pulumi_wasm_rust::Output<String>,
        /// The zone identifier to target for the resource.
        pub zone_id: pulumi_wasm_rust::Output<String>,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(name: &str, args: EmailRoutingRuleArgs) -> EmailRoutingRuleResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let actions_binding = args.actions.get_inner();
        let enabled_binding = args.enabled.get_inner();
        let matchers_binding = args.matchers.get_inner();
        let name_binding = args.name.get_inner();
        let priority_binding = args.priority.get_inner();
        let zone_id_binding = args.zone_id.get_inner();
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
            results: Vec::from([
                register_interface::ResultField {
                    name: "actions".into(),
                },
                register_interface::ResultField {
                    name: "enabled".into(),
                },
                register_interface::ResultField {
                    name: "matchers".into(),
                },
                register_interface::ResultField {
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "priority".into(),
                },
                register_interface::ResultField {
                    name: "tag".into(),
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
        EmailRoutingRuleResult {
            actions: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("actions").unwrap(),
            ),
            enabled: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("enabled").unwrap(),
            ),
            matchers: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("matchers").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            priority: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("priority").unwrap(),
            ),
            tag: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("tag").unwrap(),
            ),
            zone_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("zoneId").unwrap(),
            ),
        }
    }
}
