pub struct FirewallRuleArgs {
    pub action: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub filter_id: pulumi_wasm_rust::Output<String>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct FirewallRuleResult {
    pub action: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub filter_id: pulumi_wasm_rust::Output<String>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub products: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: FirewallRuleArgs) -> FirewallRuleResult {
    let result = crate::bindings::pulumi::cloudflare::firewall_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::firewall_rule::Args {
            action: args.action.get_inner(),
            description: args.description.get_inner(),
            filter_id: args.filter_id.get_inner(),
            paused: args.paused.get_inner(),
            priority: args.priority.get_inner(),
            products: args.products.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    FirewallRuleResult {
        action: crate::into_domain(result.action),
        description: crate::into_domain(result.description),
        filter_id: crate::into_domain(result.filter_id),
        paused: crate::into_domain(result.paused),
        priority: crate::into_domain(result.priority),
        products: crate::into_domain(result.products),
        zone_id: crate::into_domain(result.zone_id),
    }
}
