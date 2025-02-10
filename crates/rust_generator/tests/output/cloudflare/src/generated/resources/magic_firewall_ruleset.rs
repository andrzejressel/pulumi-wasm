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
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub mod magic_firewall_ruleset {
    #[derive(pulumi_gestalt_rust::__private::bon::Builder)]
    #[builder(finish_fn = build_struct)]
    #[allow(dead_code)]
    pub struct MagicFirewallRulesetArgs {
        /// The ID of the account where the ruleset is being created.
        #[builder(into)]
        pub account_id: pulumi_gestalt_rust::InputOrOutput<String>,
        /// A note that can be used to annotate the rule.
        #[builder(into, default)]
        pub description: pulumi_gestalt_rust::InputOrOutput<Option<String>>,
        /// The name of the ruleset.
        #[builder(into)]
        pub name: pulumi_gestalt_rust::InputOrOutput<String>,
        #[builder(into, default)]
        pub rules: pulumi_gestalt_rust::InputOrOutput<
            Option<Vec<std::collections::HashMap<String, String>>>,
        >,
    }
    #[allow(dead_code)]
    pub struct MagicFirewallRulesetResult {
        /// The ID of the account where the ruleset is being created.
        pub account_id: pulumi_gestalt_rust::Output<String>,
        /// A note that can be used to annotate the rule.
        pub description: pulumi_gestalt_rust::Output<Option<String>>,
        /// The name of the ruleset.
        pub name: pulumi_gestalt_rust::Output<String>,
        pub rules: pulumi_gestalt_rust::Output<
            Option<Vec<std::collections::HashMap<String, String>>>,
        >,
    }
    ///
    /// Registers a new resource with the given unique name and arguments
    ///
    #[allow(non_snake_case, unused_imports, dead_code)]
    pub fn create(
        context: &pulumi_gestalt_rust::Context,
        name: &str,
        args: MagicFirewallRulesetArgs,
    ) -> MagicFirewallRulesetResult {
        use std::collections::HashMap;
        use pulumi_gestalt_rust::{GestaltCompositeOutput, GestaltContext, GestaltOutput};
        let account_id_binding = args.account_id.get_output(context);
        let description_binding = args.description.get_output(context);
        let name_binding = args.name.get_output(context);
        let rules_binding = args.rules.get_output(context);
        let request = pulumi_gestalt_rust::RegisterResourceRequest {
            type_: "cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset".into(),
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
                    name: "name".into(),
                    value: name_binding.get_id(),
                },
                pulumi_gestalt_rust::ResourceRequestObjectField {
                    name: "rules".into(),
                    value: rules_binding.get_id(),
                },
            ],
        };
        let o = context.register_resource(request);
        MagicFirewallRulesetResult {
            account_id: o.get_field("accountId"),
            description: o.get_field("description"),
            name: o.get_field("name"),
            rules: o.get_field("rules"),
        }
    }
}
