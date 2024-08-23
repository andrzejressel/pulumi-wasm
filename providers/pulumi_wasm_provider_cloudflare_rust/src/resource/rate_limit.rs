pub struct RateLimitArgs {
    pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
    pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub match_: pulumi_wasm_rust::Output<Option<crate::types::RateLimitMatch>>,
    pub period: pulumi_wasm_rust::Output<i32>,
    pub threshold: pulumi_wasm_rust::Output<i32>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct RateLimitResult {
    pub action: pulumi_wasm_rust::Output<crate::types::RateLimitAction>,
    pub bypass_url_patterns: pulumi_wasm_rust::Output<Option<Vec<String>>>,
    pub correlate: pulumi_wasm_rust::Output<Option<crate::types::RateLimitCorrelate>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub disabled: pulumi_wasm_rust::Output<Option<bool>>,
    pub match_: pulumi_wasm_rust::Output<crate::types::RateLimitMatch>,
    pub period: pulumi_wasm_rust::Output<i32>,
    pub threshold: pulumi_wasm_rust::Output<i32>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: RateLimitArgs) -> RateLimitResult {
    let result = crate::bindings::pulumi::cloudflare::rate_limit::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::rate_limit::Args {
            action: args.action.get_inner(),
            bypass_url_patterns: args.bypass_url_patterns.get_inner(),
            correlate: args.correlate.get_inner(),
            description: args.description.get_inner(),
            disabled: args.disabled.get_inner(),
            match_: args.match_.get_inner(),
            period: args.period.get_inner(),
            threshold: args.threshold.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    RateLimitResult {
        action: crate::into_domain(result.action),
        bypass_url_patterns: crate::into_domain(result.bypass_url_patterns),
        correlate: crate::into_domain(result.correlate),
        description: crate::into_domain(result.description),
        disabled: crate::into_domain(result.disabled),
        match_: crate::into_domain(result.match_),
        period: crate::into_domain(result.period),
        threshold: crate::into_domain(result.threshold),
        zone_id: crate::into_domain(result.zone_id),
    }
}
