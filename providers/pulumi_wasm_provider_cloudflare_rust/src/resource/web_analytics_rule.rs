pub struct WebAnalyticsRuleArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub host: pulumi_wasm_rust::Output<String>,
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}

pub struct WebAnalyticsRuleResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub host: pulumi_wasm_rust::Output<String>,
    pub inclusive: pulumi_wasm_rust::Output<bool>,
    pub is_paused: pulumi_wasm_rust::Output<bool>,
    pub paths: pulumi_wasm_rust::Output<Vec<String>>,
    pub ruleset_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WebAnalyticsRuleArgs) -> WebAnalyticsRuleResult {
    let result = crate::bindings::pulumi::cloudflare::web_analytics_rule::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::web_analytics_rule::Args {
            account_id: args.account_id.get_inner(),
            host: args.host.get_inner(),
            inclusive: args.inclusive.get_inner(),
            is_paused: args.is_paused.get_inner(),
            paths: args.paths.get_inner(),
            ruleset_id: args.ruleset_id.get_inner(),
        },
    );

    WebAnalyticsRuleResult {
        account_id: crate::into_domain(result.account_id),
        host: crate::into_domain(result.host),
        inclusive: crate::into_domain(result.inclusive),
        is_paused: crate::into_domain(result.is_paused),
        paths: crate::into_domain(result.paths),
        ruleset_id: crate::into_domain(result.ruleset_id),
    }
}
