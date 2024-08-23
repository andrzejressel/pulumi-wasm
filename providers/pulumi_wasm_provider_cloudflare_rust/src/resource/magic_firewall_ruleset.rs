pub struct MagicFirewallRulesetArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
}

pub struct MagicFirewallRulesetResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<std::collections::HashMap<String, String>>>>,
}

pub fn create(name: &str, args: MagicFirewallRulesetArgs) -> MagicFirewallRulesetResult {
    let result = crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::magic_firewall_ruleset::Args {
            account_id: args.account_id.get_inner(),
            description: args.description.get_inner(),
            name: args.name.get_inner(),
            rules: args.rules.get_inner(),
        },
    );

    MagicFirewallRulesetResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        name: crate::into_domain(result.name),
        rules: crate::into_domain(result.rules),
    }
}
