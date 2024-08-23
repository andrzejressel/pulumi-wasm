pub struct TeamsRuleArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub action: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub device_posture: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub identity: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub precedence: pulumi_wasm_rust::Output<i32>,
    pub rule_settings: pulumi_wasm_rust::Output<Option<crate::types::TeamsRuleRuleSettings>>,
    pub traffic: pulumi_wasm_rust::Output<Option<String>>,
}

pub struct TeamsRuleResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub action: pulumi_wasm_rust::Output<String>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub device_posture: pulumi_wasm_rust::Output<Option<String>>,
    pub enabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub filters: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub identity: pulumi_wasm_rust::Output<Option<String>>,
    pub name: pulumi_wasm_rust::Output<String>,
    pub precedence: pulumi_wasm_rust::Output<i32>,
    pub rule_settings: pulumi_wasm_rust::Output<Option<crate::types::TeamsRuleRuleSettings>>,
    pub traffic: pulumi_wasm_rust::Output<Option<String>>,
    pub version: pulumi_wasm_rust::Output<i32>,
}

pub fn create(name: &str, args: TeamsRuleArgs) -> TeamsRuleResult {
    let result = crate::bindings::pulumi::cloudflare::teams_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::teams_rule::Args {
            account_id: args.account_id.get_inner(),
            action: args.action.get_inner(),
            description: args.description.get_inner(),
            device_posture: args.device_posture.get_inner(),
            enabled: args.enabled.get_inner(),
            filters: args.filters.get_inner(),
            identity: args.identity.get_inner(),
            name: args.name.get_inner(),
            precedence: args.precedence.get_inner(),
            rule_settings: args.rule_settings.get_inner(),
            traffic: args.traffic.get_inner(),
        },
    );

    TeamsRuleResult {
        account_id: crate::into_domain(result.account_id),
        action: crate::into_domain(result.action),
        description: crate::into_domain(result.description),
        device_posture: crate::into_domain(result.device_posture),
        enabled: crate::into_domain(result.enabled),
        filters: crate::into_domain(result.filters),
        identity: crate::into_domain(result.identity),
        name: crate::into_domain(result.name),
        precedence: crate::into_domain(result.precedence),
        rule_settings: crate::into_domain(result.rule_settings),
        traffic: crate::into_domain(result.traffic),
        version: crate::into_domain(result.version),
    }
}
