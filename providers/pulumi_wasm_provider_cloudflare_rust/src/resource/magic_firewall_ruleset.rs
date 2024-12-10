//! Magic Firewall is a network-level firewall to protect networks that are onboarded to Cloudflare's Magic Transit. This resource
//! creates a root ruleset on the account level and contains one or more rules. Rules can be crafted in Wireshark syntax and
//! are evaluated in order, with the first rule having the highest priority.
//! 
//! ## Example Usage
//! 
//! ```yaml
//! resources:
//!   example:
//!     type: cloudflare:MagicFirewallRuleset
//!     properties:
//!       accountId: d41d8cd98f00b204e9800998ecf8427e
//!       name: Magic Transit Ruleset
//!       description: Global mitigations
//!       rules:
//!         - action: allow
//!           expression: tcp.dstport in { 32768..65535 }
//!           description: Allow TCP Ephemeral Ports
//!           enabled: 'true'
//!         - action: block
//!           expression: ip.len >= 0
//!           description: Block all
//!           enabled: 'true'
//! ```
//! 
//! ## Import
//! 
//! An existing Magic Firewall Ruleset can be imported using the account ID and ruleset ID
//! 
//! ```sh
//! $ pulumi import cloudflare:index/magicFirewallRuleset:MagicFirewallRuleset example d41d8cd98f00b204e9800998ecf8427e/cb029e245cfdd66dc8d2e570d5dd3322
//! ```

#[derive(bon::Builder)]
#[builder(finish_fn = build_struct)]
pub struct MagicFirewallRulesetArgs {
    /// The ID of the account where the ruleset is being created.
    #[builder(into)]
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A note that can be used to annotate the rule.
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the ruleset.
    #[builder(into)]
    pub name: pulumi_wasm_rust::Output<String>,
    #[builder(into, default = ::pulumi_wasm_rust::Output::empty())]
    pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
}

pub struct MagicFirewallRulesetResult {
    /// The ID of the account where the ruleset is being created.
    pub account_id: pulumi_wasm_rust::Output<String>,
    /// A note that can be used to annotate the rule.
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    /// The name of the ruleset.
    pub name: pulumi_wasm_rust::Output<String>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
}

///
/// Registers a new resource with the given unique name and arguments
///
pub fn create(name: &str, args: MagicFirewallRulesetArgs) -> MagicFirewallRulesetResult {

    let result = crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::invoke(name, &crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::Args {
        account_id: &args.account_id.get_inner(),
        description: &args.description.get_inner(),
        name: &args.name.get_inner(),
        rules: &args.rules.get_inner(),
    });

    MagicFirewallRulesetResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        name: crate::into_domain(result.name),
        rules: crate::into_domain(result.rules),
    }
}

