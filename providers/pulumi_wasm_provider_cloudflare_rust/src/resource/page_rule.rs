pub struct PageRuleArgs {
    pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    pub target: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct PageRuleResult {
    pub actions: pulumi_wasm_rust::Output<crate::types::PageRuleActions>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub status: pulumi_wasm_rust::Output<Option<String>>,
    pub target: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: PageRuleArgs) -> PageRuleResult {
    let result = crate::bindings::pulumi::cloudflare::page_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::page_rule::Args {
            actions: args.actions.get_inner(),
            priority: args.priority.get_inner(),
            status: args.status.get_inner(),
            target: args.target.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    PageRuleResult {
        actions: crate::into_domain(result.actions),
        priority: crate::into_domain(result.priority),
        status: crate::into_domain(result.status),
        target: crate::into_domain(result.target),
        zone_id: crate::into_domain(result.zone_id),
    }
}
