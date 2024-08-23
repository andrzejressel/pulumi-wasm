pub struct EmailRoutingRuleArgs {
    pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct EmailRoutingRuleResult {
    pub actions: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleAction>>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub matchers: pulumi_wasm_rust::Output<Option<Vec<crate::types::EmailRoutingRuleMatcher>>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub priority: pulumi_wasm_rust::Output<i32>,
    pub tag: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: EmailRoutingRuleArgs) -> EmailRoutingRuleResult {
    let result = crate::bindings::pulumi::cloudflare::email_routing_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::email_routing_rule::Args {
            actions: args.actions.get_inner(),
            enabled: args.enabled.get_inner(),
            matchers: args.matchers.get_inner(),
            name: args.name.get_inner(),
            priority: args.priority.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    EmailRoutingRuleResult {
        actions: crate::into_domain(result.actions),
        enabled: crate::into_domain(result.enabled),
        matchers: crate::into_domain(result.matchers),
        name: crate::into_domain(result.name),
        priority: crate::into_domain(result.priority),
        tag: crate::into_domain(result.tag),
        zone_id: crate::into_domain(result.zone_id),
    }
}
