pub struct WorkerRouteArgs {
    pub pattern: pulumi_wasm_rust::Output<String>,
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WorkerRouteResult {
    pub pattern: pulumi_wasm_rust::Output<String>,
    pub script_name: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WorkerRouteArgs) -> WorkerRouteResult {
    let result = crate::bindings::pulumi::cloudflare::worker_route::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::worker_route::Args {
            pattern: args.pattern.get_inner(),
            script_name: args.script_name.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    WorkerRouteResult {
        pattern: crate::into_domain(result.pattern),
        script_name: crate::into_domain(result.script_name),
        zone_id: crate::into_domain(result.zone_id),
    }
}
