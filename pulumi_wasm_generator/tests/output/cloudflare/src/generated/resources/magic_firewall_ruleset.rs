/// Magic Firewall is a network-level firewall to protect networks that are onboarded to Cloudflare's Magic Transit. This resource
/// creates a root ruleset on the account level and contains one or more rules. Rules can be crafted in Wireshark syntax and
/// are evaluated in order, with the first rule having the highest priority.
///
/// ## Example Usage
///
/// ```yaml
/// resources:
///   example:
///     type: cloudflare:MagicFirewallRuleset
///     properties:
///       accountId: d41d8cd98f00b204e9800998ecf8427e
///       name: Magic Transit Ruleset
///       description: Global mitigations
///       rules:
///         - action: allow
///           expression: tcp.dstport in { 32768..65535 }
///           description: Allow TCP Ephemeral Ports
///           enabled: 'true'
///         - action: block
///           expression: ip.len >= 0
///           description: Block all
///           enabled: 'true'
/// ```
///
/// ## Import
///
/// An existing Magic Firewall Ruleset can be imported using the account ID and ruleset ID
///
/// ```sh
/// $ pulumi import cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset example d41d8cd98f00b204e9800998ecf8427e/cb029e245cfdd66dc8d2e570d5dd3322
/// ```
pub mod magic_firewall_ruleset {
    #[derive(pulumi_wasm_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MagicFirewallRulesetArgs {
        /// The ID of the account where the ruleset is being created.
        #[builder(into)]
        pub account_id: pulumi_wasm_rust::InputOrOutput<String>,
        /// A note that can be used to annotate the rule.
        #[builder(into, default)]
        pub description: pulumi_wasm_rust::InputOrOutput<Option<String>>,
        /// The name of the ruleset.
        #[builder(into)]
        pub name: pulumi_wasm_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub rules: pulumi_wasm_rust::InputOrOutput<
            Option<Vec<std::collections::HashMap<String, String>>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MagicFirewallRulesetResult {
        /// The ID of the account where the ruleset is being created.
        pub account_id: pulumi_wasm_rust::Output<String>,
        /// A note that can be used to annotate the rule.
        pub description: pulumi_wasm_rust::Output<Option<String>>,
        /// The name of the ruleset.
        pub name: pulumi_wasm_rust::Output<String>,
        pub rules: pulumi_wasm_rust::Output<
            Option<Vec<std::collections::HashMap<String, String>>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_wasm_rust::PulumiContext,
        name: &str,
        args: MagicFirewallRulesetArgs,
    ) -> MagicFirewallRulesetResult {
        use pulumi_wasm_rust::__private::pulumi_wasm_wit::client_bindings::component::pulumi_wasm::register_interface;
        use std::collections::HashMap;
        let account_id_binding = args.account_id.get_output(context).get_inner();
        let description_binding = args.description.get_output(context).get_inner();
        let name_binding = args.name.get_output(context).get_inner();
        let rules_binding = args.rules.get_output(context).get_inner();
        let request = register_interface::RegisterResourceRequest {
            type_: "cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset".into(),
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
                    name: "name".into(),
                    value: &name_binding,
                },
                register_interface::ObjectField {
                    name: "rules".into(),
                    value: &rules_binding,
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
                    name: "name".into(),
                },
                register_interface::ResultField {
                    name: "rules".into(),
                },
            ]),
        };
        let o = register_interface::register(context.get_inner(), &request);
        let mut hashmap: HashMap<String, _> = o
            .fields
            .into_iter()
            .map(|f| (f.name, f.output))
            .collect();
        MagicFirewallRulesetResult {
            account_id: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("accountId").unwrap(),
            ),
            description: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("description").unwrap(),
            ),
            name: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("name").unwrap(),
            ),
            rules: pulumi_wasm_rust::__private::into_domain(
                hashmap.remove("rules").unwrap(),
            ),
        }
    }
}
