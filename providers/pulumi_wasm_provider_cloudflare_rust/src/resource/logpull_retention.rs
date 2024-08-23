pub struct LogpullRetentionArgs {
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct LogpullRetentionResult {
    pub enabled: pulumi_wasm_rust::Output<bool>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: LogpullRetentionArgs) -> LogpullRetentionResult {
    let result = crate::bindings::pulumi::cloudflare::logpull_retention::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::logpull_retention::Args {
            enabled: args.enabled.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    LogpullRetentionResult {
        enabled: crate::into_domain(result.enabled),
        zone_id: crate::into_domain(result.zone_id),
    }
}
