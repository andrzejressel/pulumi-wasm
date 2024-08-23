pub struct ArgoArgs {
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub struct ArgoResult {
    pub smart_routing: pulumi_wasm_rust::Output<Option<String>>,
    pub tiered_caching: pulumi_wasm_rust::Output<Option<String>>,
    pub zone_id: pulumi_wasm_rust::Output<String>,
}

pub fn create(name: &str, args: ArgoArgs) -> ArgoResult {
    let result = crate::bindings::pulumi::cloudflare::argo::invoke(
        name,
        &crate::bindings::pulumi::cloudflare::argo::Args {
            smart_routing: args.smart_routing.get_inner(),
            tiered_caching: args.tiered_caching.get_inner(),
            zone_id: args.zone_id.get_inner(),
        },
    );

    ArgoResult {
        smart_routing: crate::into_domain(result.smart_routing),
        tiered_caching: crate::into_domain(result.tiered_caching),
        zone_id: crate::into_domain(result.zone_id),
    }
}
