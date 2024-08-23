pub struct UserAgentBlockingRuleArgs {
    pub configuration: pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub paused: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct UserAgentBlockingRuleResult {
    pub configuration: pulumi_wasm_rust::Output<crate::types::UserAgentBlockingRuleConfiguration>,
    pub description: pulumi_wasm_rust::Output<String>,
    pub mode: pulumi_wasm_rust::Output<String>,
    pub paused: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: UserAgentBlockingRuleArgs) -> UserAgentBlockingRuleResult {
    let result = crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::user_agent_blocking_rule::Args {
            configuration: args.configuration.get_inner(),
            description: args.description.get_inner(),
            mode: args.mode.get_inner(),
            paused: args.paused.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    UserAgentBlockingRuleResult {
        configuration: crate::into_domain(result.configuration),
        description: crate::into_domain(result.description),
        mode: crate::into_domain(result.mode),
        paused: crate::into_domain(result.paused),
        zone_id: crate::into_domain(result.zone_id),
    }
}
