pub struct WorkerDomainArgs {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub service: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct WorkerDomainResult {
    pub account_id: pulumi_wasm_rust::Output<String>,
    pub environment: pulumi_wasm_rust::Output<Option<String>>,
    pub hostname: pulumi_wasm_rust::Output<String>,
    pub service: pulumi_wasm_rust::Output<String>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: WorkerDomainArgs) -> WorkerDomainResult {
    let result = crate::bindings::pulumi::cloudflare::worker_domain::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::worker_domain::Args {
            account_id: args.account_id.get_inner(),
            environment: args.environment.get_inner(),
            hostname: args.hostname.get_inner(),
            service: args.service.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    WorkerDomainResult {
        account_id: crate::into_domain(result.account_id),
        environment: crate::into_domain(result.environment),
        hostname: crate::into_domain(result.hostname),
        service: crate::into_domain(result.service),
        zone_id: crate::into_domain(result.zone_id),
    }
}
