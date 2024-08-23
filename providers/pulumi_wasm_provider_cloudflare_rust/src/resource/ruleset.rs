pub struct RulesetArgs {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub kind: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub phase: pulumi_wasm_rust::Output<String>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct RulesetResult {
    pub account_id: pulumi_wasm_rust::Output<Option<String>>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub kind: pulumi_wasm_rust::Output<String>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub phase: pulumi_wasm_rust::Output<String>,
    pub rules: pulumi_wasm_rust::Output<Option<Vec<crate::types::RulesetRule>>>,
    pub zone_id: pulumi_wasm_rust::Output<Option<String>>,
}

pub fn create(name: &str, args: RulesetArgs) -> RulesetResult {
    let result = crate::bindings::pulumi::cloudflare::ruleset::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::ruleset::Args {
            account_id: args.account_id.get_inner(),
            description: args.description.get_inner(),
            kind: args.kind.get_inner(),
            name: args.name.get_inner(),
            phase: args.phase.get_inner(),
            rules: args.rules.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    RulesetResult {
        account_id: crate::into_domain(result.account_id),
        description: crate::into_domain(result.description),
        kind: crate::into_domain(result.kind),
        name: crate::into_domain(result.name),
        phase: crate::into_domain(result.phase),
        rules: crate::into_domain(result.rules),
        zone_id: crate::into_domain(result.zone_id),
    }
}
