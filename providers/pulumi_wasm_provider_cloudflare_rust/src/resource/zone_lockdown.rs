pub struct ZoneLockdownArgs {
    pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ZoneLockdownResult {
    pub configurations: pulumi_wasm_rust::Output<Vec<crate::types::ZoneLockdownConfiguration>>,
    pub description: pulumi_wasm_rust::Output<Option<String>>,
    pub paused: pulumi_wasm_rust::Output<Option<bool>>,
    pub priority: pulumi_wasm_rust::Output<Option<i32>>,
    pub urls: pulumi_wasm_rust::Output<Vec<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ZoneLockdownArgs) -> ZoneLockdownResult {
    let result = crate::bindings::pulumi::cloudflare::zone_lockdown::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::zone_lockdown::Args {
            configurations: args.configurations.get_inner(),
            description: args.description.get_inner(),
            paused: args.paused.get_inner(),
            priority: args.priority.get_inner(),
            urls: args.urls.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ZoneLockdownResult {
        configurations: crate::into_domain(result.configurations),
        description: crate::into_domain(result.description),
        paused: crate::into_domain(result.paused),
        priority: crate::into_domain(result.priority),
        urls: crate::into_domain(result.urls),
        zone_id: crate::into_domain(result.zone_id),
    }
}
